<template>
  <div id="app">
    <div class="sidebar">
      <router-link to="/chat">Chat</router-link>
      <router-link to="/prompt">Prompt</router-link>
      <router-link to="/model">Model</router-link>
      <router-link to="/plugins">Plugins</router-link>
      <router-link to="/settings">Settings</router-link>
    </div>
    <div class="main-content">
      <router-view></router-view>
    </div>
  </div>
</template>

<script>
export default {
  // Add component logic here
}
</script>

<style>
#app {
  display: flex;
}

.sidebar {
  width: 40px;
  height: 80vh;
  background-color: #f0f0f0;
  padding: 10px;
}

.sidebar a {
  display: block;
  margin-bottom: 10px;
}

.main-content {
  flex-grow: 1;
  padding: 20px;
  /* Add main content styles here */
}
</style>
