// https://stackoverflow.com/a/54165192/5322506

import axios from 'axios'

import { isProduction, env } from '@/utils/env'

let http = null // not possible to create a private property in JavaScript, so we move it outside of the class, so that it's only accessible within this module

class APIProvider {
    constructor ({ url }) {
        http = axios.create({
            baseURL: url,
            headers: { 'Content-Type': 'application/json' }
        })
    }

    login (token) {
        http.defaults.headers.common.Authorization = `Bearer ${token}`
    }

    logout () {
        http.defaults.headers.common.Authorization = ''
    }

    // REST Methods
    find ({ resource, query }) {
        return http.get(resource, {
            params: query
        })
    }

    get ({ resource, id, query }) {
        return http.get(`${resource}/${id}`, {
            params: query
        })
    }

    postMessage({message}){
        return http.post('/message', message)
    }

    create ({ resource, data, query }) {
        return http.post(resource, data, {
            params: query
        })
    }

    update ({ resource, id, data, query }) {
        return http.patch(`${resource}/${id}`, data, {
            params: query
        })
    }

    destroy ({ resource, id }) {
        return http.delete(`${resource}/${id}`)
    }
}

export default new APIProvider({
    url: env('API_URL')  // We assume 'https://api.example.com/v1' is set as the env variable
})