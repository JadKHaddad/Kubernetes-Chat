<template>
  <div class="app">
    <div class="connection-banner" :class="{ connected: connected }"></div>
    <div class="content" :class="{ 'no-padding full-height': mobileVeiw }">
      <router-view
        :mobileVeiw="mobileVeiw"
        :contacts="contacts"
        :selectedContact="selectedContact"
      />

      <!-- add contact modal -->
      <div id="add-contact-modal" uk-modal ref="add-contact-modal">
        <div class="uk-modal-dialog uk-modal-body">
          <div class="add-contact-container">
            <div class="uk-margin">
              <div class="uk-inline sign-in-form">
                <span class="uk-form-icon" uk-icon="icon: user"></span>
                <input
                  class="uk-input"
                  type="text"
                  v-model="newContactUsername"
                  ref="new-contact-username"
                  v-on:keyup.enter="addContact"
                />
              </div>
            </div>
            <p class="uk-text-right">
              <button
                class="uk-button uk-button-primary uk-modal-close"
                type="button"
                @click="addContact"
              >
                <!--@click="signin"-->
                Add
              </button>
            </p>
          </div>
        </div>
      </div>
      <!-- Notification modal -->
      <div id="notification-modal" uk-modal ref="notification-modal">
        <div class="uk-modal-dialog uk-modal-body">new message</div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: "App",
  data() {
    return {
      mobileVeiw: /Android|webOS|iPhone|iPad|iPod|BlackBerry/i.test(
        navigator.userAgent
      ),
      newContactUsername: "",
      connected: false,
      selectedContact: { username: null, info: null },
      contacts: {
        herbert: {
          status: "online",
          typing: false,
          messages: [
            {
              type: "txt",
              received: true,
              date: "18:19",
              text_content: "hi dude wassap",
              id: "qwe3",
            },

            {
              type: "img",
              imagePath:
                "https://assets.digitalocean.com/articles/alligator/css/object-fit/example-object-fit.jpg",
              received: true,
              date: "18:19",
              text_content: "hi dude wassap",
            },
            {
              type: "txt",
              received: false,
              date: "18:19",
              text_content: "wassap!",
              seen: false,
              delivered: true,
              sent: true,
              id: "32",
              like: true,
            },
          ],
        },
        jakson: {
          status: "away",
          typing: false,
          typing_timer: null,
          messages: [
            {
              type: "txt",
              received: true,
              date: "18:19",
              text_content: "hi dude wassap",
            },
            {
              type: "txt",
              received: false,
              date: "18:19",
              text_content: "yes nice dude I know!",
              seen: false,
              delivered: false,
              sent: true,
              like: true,
            },
          ],
        },
      },
    };
  },
  methods: {
    setSelectedContact(contact) {
      this.selectedContact = contact;
    },
    connenctWebsocket() {
      this.ws = new WebSocket(`ws://${location.host}/api/ws`); //process.env.VUE_APP_ROOT_API_WS);
      this.ws.onopen = () => {
        this.connected = true;
        // free up the Q
      };
      this.ws.onclose = () => {
        this.connected = false;
        // reconnect
        setTimeout(() => {
          this.connenctWebsocket();
        }, 3000);
      };
      this.ws.onmessage = (event) => {
        console.log(event.data);
        const data = JSON.parse(event.data);
        if (data.event_type === "msg") {
          // msg incoming
          const event_content = data.event_content;
          const username_target = event_content.username_target;
          // if user does not exist, create user
          if (!this.contacts.hasOwnProperty(username_target)) {
            this.requestAddContact(username_target);
            this.createContact(username_target);
          }
          // add msg
          this.contacts[username_target].messages.push(
            event_content.message_content
          );
          // clear typing
          if (this.contacts[username_target].typing_timer != null) {
            this.contacts[username_target].typing = false;
            clearTimeout(this.contacts[username_target].typing_timer);
          }
          // show notification
          if (
            event_content.message_content.received &&
            username_target !== this.selectedContact.username
          )
          this.showNotificationModal();
          // request status
          this.requestStatus([username_target]);
          return;
        }
        if (data.event_type === "typing") {
          // someone is typing
          const username_target = data.username_target;
          // if user does not exist, create user
          if (this.contacts.hasOwnProperty(username_target)) {
            this.contacts[username_target].typing = true;
            if (this.contacts[username_target].typing_timer != null) {
              clearTimeout(this.contacts[username_target].typing_timer);
            }
            this.contacts[username_target].typing_timer = setTimeout(() => {
              this.contacts[username_target].typing = false;
            }, 2500);
          }
          return;
        }
        if (data.event_type == "like") {
          const username_target = data.username_target;
          if (this.contacts.hasOwnProperty(username_target)) {
            const messageId = data.message_id;
            const selfLike = data.self_like;
            const message = this.contacts[username_target].messages.find(
              (msg) => msg.id === messageId
            );
            if (selfLike) {
              message.self_like = true;
              return;
            }
            message.like = true;
          }
          return;
        }
        if (data.event_type === "status") {
          const username_target = data.username_target;
          if (this.contacts.hasOwnProperty(username_target)) {
            this.contacts[username_target].status = data.status;
          }
          return;
        }
      };
    },
    onSignedin() {
      this.$router.replace({ name: "Home" });
      this.connenctWebsocket();
    },
    onSignedout() {
      this.$router.replace({ name: "Signin" });
      if (this.connected) {
        this.ws.close();
      }
    },
    isSignedin() {
      fetch("/api/isSignedin", { method: "POST" })
        .then((data) => data.json())
        .then((data) => {
          if (data.success) {
            this.onSignedin();
          } else {
            this.onSignedout();
          }
        })
        .catch();
    },
    signin(email, password) {
      fetch("/api/signin", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ email: email, password: password }),
      })
        .then((data) => data.json())
        .then((data) => {
          if (data.success) {
            this.onSignedin();
          } else {
            this.onSignedout();
          }
        })
        .catch();
    },
    signup(email, username, password) {
      fetch("/api/signup", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          email: email,
          username: username,
          password: password,
        }),
      })
        .then((data) => data.json())
        .then((data) => {
          if (data.success) {
            this.onSignedin();
          } else {
            this.onSignedout();
          }
        })
        .catch();
    },
    showAddContactModal() {
      UIkit.modal(this.$refs["add-contact-modal"]).show();
    },
    hideAddContactModal() {
      UIkit.modal(this.$refs["add-contact-modal"]).hide();
    },
    showNotificationModal() {
      UIkit.modal(this.$refs["notification-modal"]).show();
    },
    async requestAddContact(username) {
      return new Promise((resolve, reject) => {
        fetch("/api/addContact", {
          method: "POST",
          headers: {
          "Content-Type": "application/json",
          },
          body: JSON.stringify({ username: username }),
        })
          .then((data) => data.json())
          .then((data) => resolve(data))
          .catch((err) => reject(err));
      });
    },
    addContact() {
      this.requestAddContact(this.newContactUsername)
        .then((data) => {
          if (data.success) {
            const username = this.newContactUsername
            const newContact = this.createContact(username);
            this.setSelectedContact(newContact);
            // request status
            this.requestStatus([username]);
          } else {
          }
          this.hideAddContactModal();
          this.newContactUsername = "";
        })
        .catch();
    },
    createContact(username) {
      const newContactInfo = {
        status: "offline",
        typing: false,
        messages: [],
      };
      this.contacts[username] = newContactInfo;
      return { username: username, info: newContactInfo };
    },
    requestStatus(usernames){
      if (this.connected) {
        this.ws.send(
          JSON.stringify({
            usernames: usernames,
            type: "status_request",
          })
        );
      }
    },
    typing() {
      if (this.connected) {
        this.ws.send(
          JSON.stringify({
            username_to: this.selectedContact.username,
            type: "typing",
          })
        );
      }
    },
    sendTextMessage(textContent) {
      if (this.connected) {
        this.ws.send(
          JSON.stringify({
            username_to: this.selectedContact.username,
            type: "txt",
            text_content: textContent,
          })
        );
      }
    },
    likeMessage(MessageId) {
      if (this.connected) {
        this.ws.send(
          JSON.stringify({
            username_to: this.selectedContact.username,
            type: "like",
            message_id: MessageId,
          })
        );
      }
    },
  },
  beforeCreate() {
    if (!this.selectedContact) {
      this.$router.replace({ name: "Signin" });
    }
  },
  created() {
    this.isSignedin();
  },
  mounted() {
    let observer = new MutationObserver((mutations) => {
      for (const m of mutations) {
        const newValue = m.target.getAttribute(m.attributeName);
        if (newValue.includes("uk-open")) {
          this.$nextTick(() => {
            this.$refs["new-contact-username"].focus();
          });
        }
      }
    });

    observer.observe(this.$refs["add-contact-modal"], {
      attributes: true,
      attributeOldValue: true,
      attributeFilter: ["class"],
    });
  },
};
</script>
