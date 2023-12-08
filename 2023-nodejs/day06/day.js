#!/usr/bin/env node

// part1 sample
// const data = [
//     {time: 7, record: 9},
//     {time: 15, record: 40},
//     {time: 30, record: 200},
// ]

// part1 input
// const data = [
//     {time: 46, record: 347},
//     {time: 82, record: 1522},
//     {time: 84, record: 1406},
//     {time: 79, record: 1471},
// ]

// part2 sample
// const data =[{time: 71530, record: 940200}]

// part2 sample
const data = [{time: 46828479, record: 347152214061471}]

let ways = 1

data.forEach((race) => {
    let racewins = 0
    for (let i = 1; i < race.time; i++) {
        let speed = i
        let distance = speed*(race.time-speed)
        if (distance > race.record) {
            racewins++
        }
    }
    ways *= racewins
})

console.log("number of ways", ways)
