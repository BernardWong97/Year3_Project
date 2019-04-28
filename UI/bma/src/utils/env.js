

export function env(key) {
    switch (key) {
        case 'API_URL': return 'http://192.168.1.7:8001/app'
        // eslint-disable-next-line no-undef
        default: Console.log(`Not found: ${key}`)
    }
}