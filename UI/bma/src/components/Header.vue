<template>
    <div class="w3-panel w3-border w3-light-grey w3-center">
        <p v-on:click="mineBlock">
            <img alt="Vue logo"
                 src="../assets/rust-logo.png"
                 height="50em"
                 v-bind:class="{'w3-spin': isSpin}"
        ></p>

        <!--        <img alt="Vue logo"-->
        <!--             src="../assets/rust-logo.png"-->
        <!--             height="50em"-->
        <!--        >-->
        <div class="w3-bar w3-grey w3-center">
            <!--            <router-link to="/addressBook" class="w3-bar-item w3-button ">Address Book</router-link>-->
            <router-link to="/" class="w3-bar-item w3-button ">Home</router-link>
            <router-link to="/about" class="w3-bar-item w3-button ">About</router-link>
            <router-link to="/info" class="w3-bar-item w3-button ">Info</router-link>
        </div>
    </div>
</template>

<script>
    import axios from 'axios'

    export default {
        props: ['user'],
        data() {
            return {
                isSpin: false,
            }
        },
        methods: {
            mineBlock() {
                this.isSpin = true

                axios.get(`http://192.168.1.7:8001/app/auto`)
                    .then( res => {
                        console.log(`Miner: ${res.data.miner}`)
                        this.isSpin = false
                    })
                    .catch(err => console.log(err))
            }
        }
    }
</script>

<style>
</style>