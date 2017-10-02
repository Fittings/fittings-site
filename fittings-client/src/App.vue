<template>
  <div id="app">
    <main class="wrapper">
      <header class="header">
          <div class="header-centre">
            <a class="navigation-title" href="?reload">
            <img class="navigation-logo" src='./assets/favicon.png'/>
            <span>Cameron Milsom</span>
            </a>
              <nav class="navigation">
              <a class="navigation-link" href="#galleries">Photos</a>
              <a class="navigation-link" href="https://github.com/fittings">GitHub</a>
            </nav>
          </div>
        </header>
        <router-view></router-view>
    </main>
  </div>
</template>

<script>
export default {
  name: 'app'
}
</script>

<style src="./assets/main.css"/>

