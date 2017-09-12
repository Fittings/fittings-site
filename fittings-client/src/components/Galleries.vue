<template>
  <div class="galleries">
    <h1>Galleries</h1>
    <button class="btn" v-on:click="getGalleries()">get Galleries</button>
    <span camSpan>{{galleries}}</span>
  </div>
</template>

<script>
export default {
  data () {
    return {
      msg: 'This is th!e galleries page!',
      name: 'Cameron',
      galleries: null
    }
  },
  methods: {
    getGalleries () {
      console.log('get me')
      this.$http
      .get('gallery/all', (data) => {
        console.log('Here I am')
        console.log(data)
        this.galleries = data.get(0)
      }, response => {
        console.log(response)
      })
      console.log('doRequest')
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
