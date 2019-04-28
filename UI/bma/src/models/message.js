export class Message {
    constructor({data}){
        this.text = data.load
        this.receiver = data.receiver
        this.sender = data.sender
        this.value = data.value
    }
}

