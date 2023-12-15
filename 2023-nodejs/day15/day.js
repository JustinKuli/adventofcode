#!/usr/bin/env node

const fs = require('node:fs');

// fs.readFile('./sample.txt', (err, data) => {
fs.readFile('./input.txt', (err, data) => {
    if (err) {
        console.error(err);
    }
    
    const lines = data.toString().split('\n')

    let sum1 = 0
    let boxes = {}

    lines.forEach((line) => {
        line.split(",").forEach((str) => {
            sum1 += holidayHash(str)

            if (str.includes("=")) { // set or update
                let split = str.split("=") 
                let label = split[0]
                let focalLength = split[1]
                let box = holidayHash(label)

                let newlense = {
                    "key": label,
                    "val": focalLength
                }

                if (box in boxes) {
                    let idx = boxes[box].findIndex((x) => x.key === label)
                    if (idx === -1) { // not found
                        boxes[box].push(newlense)
                    } else {
                        boxes[box][idx].val = focalLength
                    }
                } else {
                    boxes[box] = [newlense]
                }
            } else if (str.includes("-")) { // remove if present
                let split = str.split("-") 
                let label = split[0]
                let box = holidayHash(label)

                if (box in boxes) {
                    let idx = boxes[box].findIndex((x) => x.key === label)
                    if (idx !== -1) {
                        boxes[box].splice(idx, 1)
                    }
                }
            } else {
                console.log("UNKNOWN TYPE")
            }
            // console.log("after", str, boxes)
        })
    })

    let sum2 = 0
    for (let boxNum in boxes) {
        let box = boxes[boxNum]
        box.forEach((lense, slotNum) => {
            let power = (Number(boxNum)+1) * (slotNum+1) * Number(lense.val)
            sum2 += power
        })
    }

    console.log("Part One", sum1)
    console.log("Part Two", sum2)
});

function holidayHash(str) {
    let cv = 0
    for (let i=0; i<str.length; i++) {
        let ascii = str.codePointAt(i)
        cv += ascii
        cv *= 17
        cv %= 256
    }
    return cv
}
