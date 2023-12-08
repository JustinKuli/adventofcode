#!/usr/bin/env node

const fs = require('node:fs');

fs.readFile('./sample.txt', (err, data) => {
// fs.readFile('./input.txt', (err, data) => {
    if (err) {
        console.error(err);
    }
    
    const lines = data.toString().split('\n')

    let instruction = ""
    let nodes = {}
    let positions = []

    lines.forEach((line) => {
        const parts = line.split(" ")

        if (line.length === 0) { // deal with the empty line
            return
        }

        if (parts.length === 1) {
            instruction = parts[0].split("")
        } else {
            nodes[parts[0]] = {
                L: parts[2].slice(1,4),
                R: parts[3].slice(0,3)
            }

            if (parts[0].slice(2) === "A") {
                positions.push(parts[0])
            }
        }
    })

    let loops = []

    positions.forEach((pos) => {
        let step = 0

        while (true) {
            let instruct = instruction[step % instruction.length]
            pos = nodes[pos][instruct]
            step++

            if (pos.slice(2) === "Z") {
                loops.push(step)
                return
            }
        }
    })

    // positions.forEach((pos) => {
    //     let step = 0
    //     let visited = {}
    //     let found = 0

    //     while (true) {
    //         let instruct = instruction[step % instruction.length] 
    //         let visitID = pos + step % instruction.length

    //         if (visitID in visited) {
    //             // loop found!
    //             let first = visited[visitID]

    //             loops.push({offset: first, period: step - first})
    //             return
    //         }

    //         visited[visitID] = step
    //         pos = nodes[pos][instruct]
    //         step++
    //     }
    // })

    console.log("Part answer:", loops)
});
