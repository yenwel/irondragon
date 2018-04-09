<template>
  <div class="hello">
    <md-content class="md-elevation-3">
      <div class="md-body-2">{{ msg }}</div>
      <div class="actions">
        <md-button class="md-raised md-primary" @click="moveWings">Move wings</md-button>
      </div>
      <div class="md-body-2">{{ wingsmsg }}</div>
    </md-content>
  </div>
</template>

<script>
import axios from 'axios'
import _ from 'lodash'

export default {
  name: 'hello',
  data () {
    return {
      msg: 'Welcome to the dragon controller',
      wingsmsg: 'Wings are ready'
    }
  },
  methods: {
    moveWings: _.debounce(() => {
      this.wingsmsg = 'Moving Wings'
      axios.post('/api/wings')
      .then(response => { this.wingsmsg = response.statusText })
      .catch(e => console.log(e))
    }, 20)
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
