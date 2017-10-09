<template>
  <div class="gallery" style="width: 100%;">
    <div class="hero">
      <img class="hero-image fade-in-anim " v-bind:src="heroPreviewUrl"/>
      <div class="hero-text">
        <h1>{{ $title }}</h1>
      </div>
    </div>
    <div class="main-content">
      <div class="row gallery-row" v-for="(photo, index) in photos" v-bind:key="index">
        <div class="column">
          <a class="gallery-border">
            <div class="card slide-load-anim" v-bind:tabindex="photo.tab_index">
              <div class="card-image">
                <img class="gallery-preview" v-bind:src="photo.url" v-bind:alt="photo.description"/>
              </div>
              <div class="card-content">
                <h2>{{ photo.name }}</h2>
                <p><em>{{ photo.description }}</em></p>
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
      title: '',
      msg: 'This is the galleries page!',
      photos: null,
      heroPreviewUrl: null
    }
  },
  methods: {
    getGalleries () {
      this.$http.get('/api/gallery/' + this.$route.params.id)
      .then(data => {
        this.title = data.body
        this.photos = data.body.galleries

        if (data.body.galleries !== null) {
          let randImageIndex = (Math.round(Math.random() * (data.body.galleries.length - 1)))
          this.heroPreviewUrl = data.body.galleries[randImageIndex].url
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
  .main-content {
    margin-top: -10rem;
  }
  .main-content * {
    z-index: 1;
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
    user-select: none;
  }


  .card:hover {
    box-shadow:0 3px 3px 0 rgba(0,0,0,0.14),0 1px 7px 0 rgba(0,0,0,0.12),0 3px 1px -1px rgba(0,0,0,0.2);
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
