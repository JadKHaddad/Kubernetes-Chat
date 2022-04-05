<template>
  <div class="flex-content full-height">
    <ContactList :contacts="contacts" />
    <ChatContainer v-if="contactSelected" :selectedContact="selectedContact" />
  </div>
</template>

<script>
import ContactList from "@/components/ContactList.vue";
import ChatContainer from "@/components/ChatContainer.vue";
export default {
  name: "DesktopHome",
  props: ["contacts", "selectedContact"],
  components: {
    ContactList,
    ChatContainer,
  },
  computed: {
    contactSelected() {
      return this.selectedContact.username;
    },
  },
  updated(){

  }
};
</script>

<style>
</style>