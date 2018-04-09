<template>
  <div class="hello">
    <md-content class="md-elevation-3">
      <h4>{{ msg }}</h4>
      <div class="actions md-layout md-alignment-center-space-between">
        <md-button class="md-raised md-primary" @click="moveWings">Move wings</md-button>
      </div>
      <h4>{{ wingsmsg }}</h4>
    </md-content>
  </div>
</template>

<script>
export default {
  name: 'hello',
  data () {
    return {
      msg: 'Welcome to the dragon controller',
      wingsmsg: 'Wings are ready'
    }
  },
  methods: {
    moveWings: function (command) {
      this.wingsmsg = 'Moving Wings'
    }

  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h1, h2 {
  font-weight: normal;
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
