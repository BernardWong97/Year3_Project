<template>
    <div class="w3-margin w3-card-4 w3-center">
        <h1>Info</h1>
        <table class="w3-padding-24 w3-table-all">
            <tr>
                <th>User</th>
                <td>{{this.settings.username}}</td>
            </tr>
            <tr>
                <th>Balance</th>
                <td>{{this.settings.user_balance}}</td>
            </tr>
            <tr>
                <th>Blockchain UUID</th>
                <td>{{this.settings.blockchain_uuid}}</td>
            </tr>
            <tr>
                <th>Blockchain length</th>
                <td>{{this.settings.chain_len}}</td>
            </tr>
            <tr>
                <th>Block reward</th>
                <td>{{this.settings.block_reward}}</td>
            </tr>
            <tr>
                <th>Message reward</th>
                <td>{{this.settings.message_read_reward}}</td>
            </tr>
            <tr>
                <th>Message cost</th>
                <td>{{this.settings.message_send_cost}}</td>
            </tr>
        </table>
<!--        <h1>Settings</h1>-->
<!--        <p><b>User: </b>{{this.settings.username}}</p>-->
<!--        <p><b>Blockchain uuid: </b>b>{{this.settings.blockchain_uuid}}</p>-->

    </div>
</template>

<script>
    // @ is an alias to /src
    // import HelloWorld from '@/components/HelloWorld.vue'
    import axios from 'axios';

    export default {
        name: 'Info',
        // components: {},
        data() {
            return {
                settings: {}
            // {
            //     "block_reward": 0,
            //     "blockchain_uuid": "b5fd34f0-87d7-489b-84e4-0bb972f369cc",
            //     "chain_len": 0,
            //     "message_read_reward": 0,
            //     "message_send_cost": 0,
            //     "user_balance": 8,
            //     "username": "Mindaugas"
            // }
            }
        },
        // methods: {
        //     // addTodo(newTodo) {
        //     //     const { title, completed } = newTodo;
        //     //     axios.post('https://jsonplaceholder.typicode.com/todos', {
        //     //         title,
        //     //         completed
        //     //     })
        //     //         .then(res => this.todos = [...this.todos, res.data])
        //     //         .catch(err => console.log(err));
        //     // }
        // },
        created() {
            axios.get('http://192.168.1.7:8001/app/info')
                .then(res => {
                    // eslint-disable-next-line no-console
                    console.log(res.data)
                    this.settings = res.data
                })
                // eslint-disable-next-line no-console
                .catch(err => console.log(err));
        }
    }
</script>