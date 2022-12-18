const fs = require('fs')

const alphabet = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ'.split('')
const priorityMap = new Map()
let priority = 1
for (const char of alphabet) {
    priorityMap.set(char, priority)
    priority++
}

const getLineValue = (line) => {
    let lineValue = 0
    const len = line.length / 2
    const left = line.slice(0, len)
    const right = line.slice(len)
    
    for (const char of left) {
        if (right.includes(char)) lineValue = priorityMap.get(char) ?? 0
    }
    return lineValue
}

function main() {
    fs.readFile('input.txt', 'utf-8', (err, data) => {
        if (err) throw err
        let sum = 0
        const lineArr = data.split('\n')
        for (const line of lineArr) {
            sum += getLineValue(line)
        }
        console.info(sum)
    })
}

main()