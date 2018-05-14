<template>
  <div class="hello">
    <md-content class="md-elevation-3">
      <div class="md-body-2">{{ msg }}</div>
      <div class="actions">
        <md-button class="md-raised md-primary" @click="moveWings">Move wings</md-button>
      </div>
      <div class="md-body-2">{{ wingsmsg }}</div>   
      <div class="actions">
        <md-button class="md-raised md-primary" @click="openMouth">Open Mouth</md-button>
      </div>   
      <div class="md-body-2">{{ mouthmsg }}</div>  
      <div class="actions">
        <md-button class="md-raised md-primary" @click="openEyes">Open eyes</md-button>
      </div>         
      <div class="md-body-2">{{ eyesmsg }}</div>
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
      wingsmsg: 'Wings are ready',
      mouthmsg: 'Mouth is ready',
      eyesmsg: 'Eyes are ready'
    }
  },
  methods: {
    moveWings: _.debounce(() => {
      this.wingsmsg = 'Moving Wings'
      axios.post('/api/wings')
      .then(response => { this.wingsmsg = response.statusText })
      .catch(e => console.log(e))
    }, 20),
    openMouth: _.debounce(() => {
      this.mouthmsg = 'Opening Mouth'
      axios.post('/api/mouth')
      .then(response => { this.mouthmsg = response.statusText })
      .catch(e => console.log(e))
    }, 20),
    openEyes: _.debounce(() => {
      this.eyesmsg = 'Opening eyes'
      axios.post('/api/eyes')
      .then(response => { this.eyesmsg = response.statusText })
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
