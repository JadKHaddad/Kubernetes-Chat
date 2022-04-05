<template>
    <ContactList :mobileVeiw="true"/>
</template>

<script>
import ContactList from "@/components/ContactList.vue";
export default {
  name: "MobileHome",
  components: {
    ContactList,
  },
};
</script>

<style>
</style>