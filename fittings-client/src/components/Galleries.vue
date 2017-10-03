<template>
  <div class="galleries">
    <div class="hero">
      <img class="hero-image" v-bind:src="heroPreviewUrl"/>
      <div class="hero-text">
        <h1>Photos</h1>
        <p><em>From wherever.</em></p>
      </div>
    </div>
    <div class="main-content">
      <div class="row gallery-row" v-for="(gallery, index) in galleries" v-bind:key="index">
        <div class="column">
          <a class="gallery-border">
            <div class="card">
              <div class="card-image">
                <img class="gallery-preview" v-bind:src="gallery.preview_url" v-bind:alt="gallery.description"/>
              </div>
              <div class="card-content">
                <h2>{{ gallery.name }}</h2>
                <p><em>{{ gallery.description }}</em></p>
              </div>              
              </section>
            </div>
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
    width: 100%;
  }


  .hero-image {
    filter: grayscale(40%) brightness(0.8) blur(0.05rem);;
    width: 100%;
    height: 60rem;
    object-fit: cover;
    -webkit-mask-image: -webkit-gradient(linear, left 90%, left bottom, from(rgba(0,0,0,1)), to(rgba(0,0,0,0)));
    /* box-shadow: 0 4px 100px -100px white; */
    border-bottom: 1px solid rgba(0, 0, 0, 1);

  }

  .hero-text {
    text-align: center;
    position: absolute;
    top: 0;
    left: 0;
    transform: translate(0%, 100%);
    width: 100%;
    color: white;
    text-shadow: 0.1rem 0.1rem 0.5px black;
    font-weight: 100;
    }

  .hero-text > h1 {
    font-size: 10rem;
    font-weight: 800;
    margin-bottom: 0;
  }

  .main-content {
    margin-top: -10rem;
  }
  .main-content * {
    z-index: 1;
  }

  .gallery-row {
  }

  .gallery-border {
    cursor: pointer;
  }


  .gallery-border > h2 {
    color: red;
  }

  .gallery-preview-blur {
    position: absolute;
  }
  .gallery-preview {
    margin: 0 0 0 0;
    filter: saturate(90%) ;
    width: 100%;
    height: 350px;
    object-fit: cover;
    z-index: -1;

  }

  .card:hover {
    box-shadow:0 3px 3px 0 rgba(0,0,0,0.14),0 1px 7px 0 rgba(0,0,0,0.12),0 3px 1px -1px rgba(0,0,0,0.2)
  }
  .column:hover .gallery-preview {
    filter: saturate(110%) brightness(1.1);
  }

  .gallery-description a {
    color: black !important;
  }
  .gallery-description > h2 {
    margin-top: 0;
    margin-bottom: 0;
  }

  .gallery-description > p {
    margin-top: 0;
  }



</style>
