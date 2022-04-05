<template>
  <div
    class="contanct-list-container-holder full-height"
    :class="{ 'full-width': mobileVeiw }"
  >
    <div class="contanct-list-container full-height scrollbar">
      <ul class="uk-list contact-ul">
        <ContactTile v-for="contact in contactsList" :key="contact" :contact="{username: contact[0], info: contact[1]}" @click="setSelectedContact({username: contact[0], info: contact[1]})"/>
      </ul>
    </div>
    <div class="new-btn-container" @click="addContact">
      <a href="#" class="uk-icon-button new-btn" uk-icon="plus"></a>
    </div>
  </div>
</template>

<script>
import ContactTile from "@/components/ContactTile.vue";
export default {
  name: "ContactList",
  props: ["mobileVeiw", "contacts"],
  components: {
    ContactTile,
  },
  computed: {
    contactsList() {
      return Object.entries(this.contacts);
    }
  },
  methods: {
    goToChatContainer(username){
      this.$router.push({name: "Chat", params: {username: username}})
    },
    setSelectedContact(contact){
      this.$root.setSelectedContact(contact)
      if (this.mobileVeiw){
        this.goToChatContainer(contact.username)
      }
    },
    addContact(){
      this.$root.showAddContactModal()
    }
  }
};
</script>

<style>
</style>