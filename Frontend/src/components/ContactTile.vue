<template>
  <li>
    <div class="contact-container">
      <div class="contact-avatar">
        <img
          src="https://www.w3schools.com/howto/img_avatar.png"
          alt="Avatar"
        />
      </div>
      <div class="contact-info">
        <div class="contact-name uk-text-bold">
          {{ contact.username }}
        </div>
        <div class="last-message">
          <div v-if="contact.info.typing">typing..</div>
          <div v-else>
            <!--<marquee
              direction="left"
              height="40"
              scrolldelay="400"
            >
            </marquee> -->
            {{ lastMessageContent }}
          </div>
        </div>
      </div>
    </div>
  </li>
</template>

<script>
export default {
  name: "ContactTile",
  props: ["contact"],
  data() {
    return {};
  },
  computed: {
    lastMessageContent() {
      if (this.contact.info.messages.length < 1) {
        return "";
      }
      let lastMessageContent =  this.contact.info.messages.at(-1).text_content;
      if (lastMessageContent.length > 20){
        lastMessageContent = lastMessageContent.substring(0, 20) + " ..";
      }
      return lastMessageContent;
    },
  },
  mounted() {},
};
</script>




