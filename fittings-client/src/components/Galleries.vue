<template>
  <div class="galleries">
    <h1>Galleries</h1>
    <button class="btn" v-on:click="getGalleries()">Refresh</button>
    <div id="galleries">
      <div class="row" v-for="(gallery, index) in galleries" v-bind:key="index">
        <div class="column">
          <h1>{{ gallery.name }}</h1>
          <section>{{ gallery.description }}</section>
          <img v-bind:src="gallery.preview_url" v-bind:alt="gallery.description"/>


        </div>
      </div>
    </div>
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
      this.$http.get('gallery/all')
      .then(data => {
        this.galleries = data.body.galleries
        // this.galleries.forEach(gallery => {
        //   // gallery.preview_url = require('../' + gallery.preview_url)
        // })
      }, response => {
        console.log(response)
      })
    }
  },
  created: function () {
    this.getGalleries()
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
