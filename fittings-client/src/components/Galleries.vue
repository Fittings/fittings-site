<template>
  <div class="galleries">
    <div class="hero">
      <img class="hero-image" v-bind:src="heroPreviewUrl"/>
      <div class="hero-text">
        <h1>My Photos</h1>
        <p><em>From places</em></p>
      </div>
    </div>
    <div class="main-content">
      <div class="row gallery-row" v-for="(gallery, index) in galleries" v-bind:key="index">
        <div class="column">
          <a class="gallery-border">
            <img class="gallery-preview" v-bind:src="gallery.preview_url" v-bind:alt="gallery.description"/>
            <section class="gallery-description">
              <h2>{{ gallery.name }}</h2>
              <p><em>{{ gallery.description }}</em></p>
            </section>
          </a>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  data () {
    return {
      msg: 'This is the galleries page!',
      name: 'Cameron',
      galleries: null,
      heroPreviewUrl: null
    }
  },
  methods: {
    getGalleries () {
      this.$http.get('gallery/all')
      .then(data => {
        this.galleries = data.body.galleries
        if (this.galleries !== null) {
          let randImageIndex = (Math.round(Math.random() * this.galleries.length))
          this.heroPreviewUrl = this.galleries[randImageIndex].preview_url
        }
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
  .hero {
    margin-bottom: 2rem;
    box-shadow: 0 -1000px rgba(255, 255, 255, 1) inset;
    box-shadow: 0 5px -10px rgba(182, 182, 182, 0.75);
  }

  .hero-image {
    filter: grayscale(80%) brightness(0.8) blur(0.05rem);;
    width: 100%;
    height: 35rem;
    object-fit: cover;
    border-bottom: 100px white;
    /* box-shadow: 0 4px 100px -100px white; */
  }



  .hero-text {
    text-align: center;
    position: absolute;
    top: 0;
    left: 0;
    transform: translate(0%, 100%);
    width: 100%;
    color: white;
    /* offset-x | offset-y | blur-radius | color */
    text-shadow: 0.1rem 0.1rem 0.5px black;
    font-weight: 300;
  }



  .gallery-row {
    margin-bottom: 3rem;
  }


  .gallery-border {
    cursor: pointer;
  }


  .gallery-border > h2 {
    color: red;
  }

  .gallery-preview {
    filter: saturate(90%) ;
    width: 100%;
    height: 350px;
    border-radius: 0.6rem;
    object-fit: cover;
    z-index: -1;

  }

  .column:hover .gallery-preview {
    filter: saturate(110%) brightness(1.1);
  }


  .gallery-description > h2 {
    margin-top: 0;
  }


</style>
