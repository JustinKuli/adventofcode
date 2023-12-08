#!/usr/bin/env node

const fs = require('node:fs');

// fs.readFile('./sample.txt', (err, data) => {
fs.readFile('./input.txt', (err, data) => {
    if (err) {
        console.error(err);
        return
    }
    
    const lines = data.toString().split('\n')

    let p1sum = 0
    let p2sum = 0

    lines.forEach((line) => {
        const data = line.split(":")
        if (!data || data.length < 1) {
            return
        }

        const id = data[0].split(" ")[1]
        const sets = data[1].split(";")

        // PART ONE

        let possible = true

        sets.forEach((set) => {
            foo(set, /(.*) red/g, 12, () => {
                possible = false
            })

            foo(set, /(.*) green/g, 13, () => {
                possible = false
            })

            foo(set, /(.*) blue/g, 14, () => {
                possible = false
            })
        })

        if (possible) {
            p1sum += Number(id)
        }

        // PART TWO

        let minred = 0
        let mingreen = 0
        let minblue = 0

        sets.forEach((set) => {
            foo(set, /(.*) red/g, minred, (count) => {
                minred = count
            })

            foo(set, /(.*) green/g, mingreen, (count) => {
                mingreen = count
            })

            foo(set, /(.*) blue/g, minblue, (count) => {
                minblue = count
            })
        })

        p2sum += minred*mingreen*minblue
    })

    console.log("Part One:", p1sum)
    console.log("Part Two:", p2sum)
});

// Naming things is hard... also this is probably not how to
// do regex, but whatever...
function foo(set, regx, gt, cb) {
    for (const match of set.matchAll(regx)) {
        if (match[1]) {
            const data = match[1].split(" ")
            const count = Number(data[data.length-1])
            if (count && count > gt) {
                cb(count)
            }
        }
    }
}
