<template>
  <div class="uk-child-width-1-2@m uk-text-center" uk-grid>
    <div class="sign-in-container">
      <div class="uk-margin">
        <div class="uk-inline sign-in-form">
          <span class="uk-form-icon" uk-icon="icon: mail"></span>
          <input class="uk-input" type="text" v-model="singinEmail" />
        </div>
      </div>
      <div class="uk-margin">
        <div class="uk-inline sign-in-form">
          <span class="uk-form-icon" uk-icon="icon: lock"></span>
          <input class="uk-input" type="password" v-model="signinPassword" />
        </div>
      </div>
      <p class="uk-text-right">
        <button
          class="uk-button uk-button-primary"
          type="button"
          ref="sign-in-btn"
        >
          <!--@click="signin"-->
          Sign in
        </button>
      </p>
    </div>
    <div class="sign-up-container">
      <div class="uk-margin">
        <div class="uk-inline sign-up-form">
          <span class="uk-form-icon" uk-icon="icon: mail"></span>
          <input class="uk-input" type="text" v-model="singupEmail" />
        </div>
      </div>
      <div class="uk-margin">
        <div class="uk-inline sign-up-form">
          <span class="uk-form-icon" uk-icon="icon: user"></span>
          <input class="uk-input" type="text" v-model="singupUsername" />
        </div>
      </div>
      <div class="uk-margin">
        <div class="uk-inline sign-up-form">
          <span class="uk-form-icon" uk-icon="icon: lock"></span>
          <input class="uk-input" type="password" v-model="signupPassword" />
        </div>
      </div>
      <p class="uk-text-right">
        <button
          class="uk-button uk-button-primary"
          type="button"
          ref="sign-up-btn"
        >
          <!--@click="signup"-->
          Sign up
        </button>
      </p>
    </div>
  </div>
</template>

<script>
import { fromEvent } from "rxjs";
import { throttleTime } from "rxjs/operators";
export default {
  name: "Signin",
  data() {
    return {
      singinEmail: "",
      signinPassword: "",
      singupEmail: "",
      singupUsername: "",
      signupPassword: "",
    };
  },
  methods: {
    signin() {
      if (this.singinEmail !== "" && this.signinPassword !== "") {
        this.$root.signin(this.singinEmail, this.signinPassword);
      }
    },
    signup() {
      if (this.singupEmail !== "" && this.singupUsername !== "" && this.signupPassword !== "") {
        this.$root.signup(this.singupEmail, this.singupUsername, this.signupPassword);
      }
    },
  },
  mounted() {
    fromEvent(this.$refs["sign-in-btn"], "click")
      .pipe(throttleTime(1000))
      .subscribe(() => this.signin());
    fromEvent(this.$refs["sign-up-btn"], "click")
      .pipe(throttleTime(1000))
      .subscribe(() => this.signup());
  },
};
</script>

<style>
</style>