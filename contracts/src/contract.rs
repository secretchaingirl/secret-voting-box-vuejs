use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier,
    StdError, StdResult, Storage,
};

use crate::msg::{ProposalResponse, HandleMsg, InitMsg, QueryMsg};
use crate::state::{config, config_read, save, may_load, State, Proposal};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    _msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State {
        owner: deps.api.canonical_address(&env.message.sender)?,
        proposal_count: 0u64,
    };

    config(&mut deps.storage).save(&state)?;

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::CreateProposal { name } => try_create_proposal(deps, env, name),
    }
}

pub fn try_create_proposal<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    name: String,
) -> StdResult<HandleResponse> {
    let proposal = Proposal {
        name: name,
        creator: deps.api.canonical_address(&env.message.sender).unwrap(),
        status: 0u64,
        num_votes: 0u64,
        tally_yes: 0u64,
        tally_no: 064,
    };

    let state = config_read(&deps.storage).load()?;
    let proposal_id = state.proposal_count + 1;

    save(&mut deps.storage, &proposal_id.to_be_bytes(), &proposal)?;

    // Update the proposal count
    config(&mut deps.storage).update(|mut state| {
        state.proposal_count = proposal_id;
        Ok(state)
    })?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&proposal_id)?),
    })
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetProposal { proposal_id } => to_binary(&query_proposal(deps, proposal_id)?),
    }
}

fn query_proposal<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    proposal_id: u64
) -> StdResult<ProposalResponse> {
    let proposal: Option<Proposal> = may_load(&deps.storage, &proposal_id.to_be_bytes())?;

    match proposal {
        Some(proposal) => {
            return Ok(ProposalResponse {
                id: proposal_id,
                name: proposal.name,
                creator: proposal.creator,
                status: proposal.status,
                num_votes: proposal.num_votes,
                tally_yes: proposal.tally_yes,
                tally_no: proposal.tally_no,
            });
        },
        None => {
                    return Err(StdError::generic_err("Proposal not found.",));
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(20, &[]);

        let msg = InitMsg {};
        let env = mock_env("creator", &coins(10, "scrt"));

        // we can just call .unwrap() to assert this was a success
        let res = init(&mut deps, env, msg).unwrap();
        assert_eq!(0, res.messages.len());

        /*
        // it worked, let's query the state
        let res = query(&deps, QueryMsg::GetProposal {}).unwrap();
        let value: MessageResponse = from_binary(&res).unwrap();
        assert_eq!(init_str, value.message);
        */
    }

    #[test]
    fn can_create_proposal() {
        let mut deps = mock_dependencies(20, &coins(2, "scrt"));

        let msg = InitMsg {};
        let env = mock_env("anyone", &coins(2, "scrt"));
        let _res = init(&mut deps, env, msg).unwrap();

        let proposal_name = String::from("Laura's Proposal");
        let env = mock_env("creator", &coins(2, "scrt"));
        let msg = HandleMsg::CreateProposal {
            name: proposal_name.clone()
        };

        let value = handle(&mut deps, env, msg).unwrap();
        match value.data {
            Some(d) => {
                let id: u64 = from_binary(&d).unwrap();
                assert_eq!(1u64, id);

                    // should get back created proposal
                let res = query(&deps, QueryMsg::GetProposal { proposal_id: id }).unwrap();
                let value: ProposalResponse = from_binary(&res).unwrap();
                assert_eq!(proposal_name, value.name);

            },
            _ => panic!("Proposal was not created."),
        };

    }
}
