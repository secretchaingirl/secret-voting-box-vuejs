<template>
  <div class="hello">
    <h1>{{ msg }}</h1>
    <p>
      This Secret Box template is your starting point for creating a Secret App (Secret Network dApp).
    </p>
    <h3>Getting Started with VueJS</h3>
      <br>
      For information on installing and working with
      <br>
      the VueJS framework check out the documentation.
    <ul>
      <li><a href="https://cli.vuejs.org" target="_blank" rel="noopener">Vue-cli</a></li>
      <li><a href="https://vuejs.org" target="_blank" rel="noopener">Vue Learn</a></li>
    </ul>
    <h3>Secret Contracts</h3>
    <ul>
      <li><a href="https://build.scrt.network/dev/developers.html" target="_blank" rel="noopener">Developer Docs</a></li>
    </ul>
  </div>
</template>

<script>
export default {
  name: 'HelloWorld',
  props: {
    msg: String
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h3 {
  margin: 40px 0 0;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}
</style>
