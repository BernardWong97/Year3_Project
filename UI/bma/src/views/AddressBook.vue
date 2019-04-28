<template>
    <div class="w3-card w3-margin w3-round-xlarge w3-center">
        <h1 class="w3-dark-grey">Address Book</h1>

        <ul v-bind:key="user.id" v-for="user in addressBook"
            class="w3-ul w3-hoverable "
        >
            <li>
                <router-link :to="{ name: 'home', params: { user}}" >{{user}}</router-link>
            </li>
        </ul>
    </div>
</template>

<script>


    export default {
        name: "addressBook",
        props: ["addressBook"],
        async created(){
            // await console.log(this.addressBook)
        }
    }
</script>

<style scoped>

</style>