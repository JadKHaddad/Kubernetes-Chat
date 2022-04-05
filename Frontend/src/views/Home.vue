<template>
    <MobileHome v-if="mobileVeiw" :contacts="contacts"/>
    <DesktopHome v-else :contacts="contacts" :selectedContact="selectedContact"/>
</template>

<script>
import DesktopHome from "@/components/DesktopHome.vue";
import MobileHome from "@/components/MobileHome.vue";
export default {
  name: "Home",
  props: ["mobileVeiw", "contacts", "selectedContact"],
  components: {
    DesktopHome,
    MobileHome
  },
  data(){
    return {
      
    }
  }
  
}
</script>

<style>

</style>