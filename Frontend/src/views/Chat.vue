<template>
  <ChatContainer :mobileVeiw="true"/>
</template>

<script>
import ChatContainer from "@/components/ChatContainer.vue";
export default {
  name: "Chat",
  components: {
    ChatContainer,
  },
};
</script>

<style>
</style>