<template>
  <div class="chat-container full-height" :class="{ 'full-width': mobileVeiw }">
    <!--<div class="chat-banner" :class="{ 'full-width fixed z-index-5': mobileVeiw }">-->
    <div class="chat-banner">
      <div class="contact-avatar">
        <img
          src="https://www.w3schools.com/howto/img_avatar.png"
          alt="Avatar"
        />
      </div>
      <div class="contact-info">
        <div class="contact-name uk-text-bold">
          {{ selectedContact.username }}
        </div>
        <div class="contact-status">
          <div v-if="selectedContact.info.typing">typing..</div>
          <div v-else>
            {{ selectedContact.info.status }}
          </div>
        </div>
      </div>
    </div>
    <div class="messages-container">
      <div id="messages-display" class="messages-display full-width scrollbar">
        <!--<MessageDisplay3D/>-->
        <ul class="uk-list">
          <Message
            v-for="message in selectedContact.info.messages"
            :key="message"
            :message="message"
          />
        </ul>
      </div>
      <div class="message-input full-width">
        <input
          class="uk-input uk-form-large"
          type="text"
          placeholder="Message"
          v-model="textContent"
          ref="message-input"
          v-on:keyup.enter="sendTextMessage"
        />
        <div class="send-btn">
          <span
            class="send-icon"
            uk-icon="icon: arrow-right; ratio: 2"
            @click="sendTextMessage"
          ></span>
        </div>
      </div>
    </div>
  </div>
</template>
<script>
import { fromEvent } from "rxjs";
import { throttleTime } from "rxjs/operators";
import Message from "@/components/Message.vue";
import MessageDisplay3D from "@/components/MessageDisplay3D.vue";
export default {
  name: "ChatContainer",
  props: ["mobileVeiw", "selectedContact"],
  components: {
    Message,
    MessageDisplay3D,
  },
  data() {
    return {
      textContent: "",
    };
  },
  methods: {
    sendTextMessage() {
      if (this.textContent.replaceAll(/\s/g, "") !== "") {
        this.$root.sendTextMessage(this.textContent);
        this.textContent = "";
      }
    },
    typing() {
      this.$root.typing();
    },
  },
  watch: {
    selectedContact: function (newVal, oldVal) {
      this.textContent = "";
      this.$refs["message-input"].focus();
    },
  },
  mounted() {
    this.$refs["message-input"].focus();
    fromEvent(this.$refs["message-input"], "keyup")
      .pipe(throttleTime(2000))
      .subscribe(() => this.typing());
  },
};
</script>