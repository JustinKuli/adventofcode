#!/usr/bin/env node

const fs = require('node:fs');

let prmemo = {}
let pcmemo = {}

// fs.readFile('./sample.txt', (err, data) => {
fs.readFile('./input.txt', (err, data) => {
    if (err) {
        console.error(err);
    }
    
    const lines = data.toString().split('\n')

    let sum1 = 0
    let sum2 = 0
    lines.forEach((line, i) => {
        const linedata = line.split(" ")
        const lengthsStr = linedata[1] 
        const records = linedata[0].split(".").filter((x) => x.length >= 1)

        let lineAns1 = possibleCovers(records, lengthsStr)
        if (lineAns1 < 1) {
            console.log("No covers found for line", i+1)
        }
        sum1 += lineAns1

        let lengthsStrUnfolded = lengthsStr
        let unfoldedRecordsRaw = linedata[0]
        for (let i = 0; i < 4; i++) {
            lengthsStrUnfolded += "," + lengthsStr
            unfoldedRecordsRaw += "?" + linedata[0]
        }
        let recordsUnfolded = unfoldedRecordsRaw.split(".").filter((x) => x.length >= 1)

        let lineAns2 = possibleCovers(recordsUnfolded, lengthsStrUnfolded)
        if (lineAns2 < 1) {
            console.log("No covers found for unfolded line", i+1)
            return
        }
        sum2+= lineAns2

        console.log("line complete", i+1)
    })

    console.log("Part One", sum1)
    console.log("Part One", sum2)
});

function possibleCovers(records, lengthsStr) {
    let memokey = records.join(":")+";"+lengthsStr
    if (memokey in pcmemo) {
        return pcmemo[memokey]
    }

    let ans = possibleCoversREAL(records, lengthsStr)
    pcmemo[memokey] = ans
    return ans
}

// returns a number
function possibleCoversREAL(records, lengthsStr) {
    dpc("enter", records, lengthsStr)

    if (records.length === 0) {
        if (lengthsStr.length !== 0) {
            dpc("return (1) 0")
            return 0
        } else {
            dpc("return (2) 1")
            return 1
        }
    }

    if (lengthsStr.length === 0) { // no more RHS to put in
        for (let r of records) {
            if (r.includes("#")) { // LHS requires something
                dpc("return (3) 0")
                return 0
            }
        }
        dpc("return (4) 1")
        return 1
    }

    let sum = 0
    let candidates = possibleRemovals(records[0], lengthsStr)
    dpc("candidates", records[0], lengthsStr, candidates)

    if (!candidates || Object.keys(candidates).length === 0) {
        dpc("return (5) 0")
        return 0
    }

    for (let removed in candidates) {
        let duplicity = candidates[removed]
        dpc("candidate loop top", removed, duplicity)

        let remainingLengthsStr = lengthsStr.replace(removed, "")
        if (remainingLengthsStr.startsWith(",")) {
            remainingLengthsStr = remainingLengthsStr.replace(",", "")
        }

        let add = possibleCovers(records.slice(1), remainingLengthsStr)
        sum += add*duplicity
        dpc("candidate loop bottom", add, duplicity, "newsum", sum)
    }

    return sum
}

function dpc(...args) {
    // console.log("covers", ...args)
}

let dopctest = true
if (dopctest) {
    pctest(["??","??"], "1,1", 4)
    pctest(["???", "###"], "1,1,3", 1)
    pctest(["??", "??", "?##"], "1,1,3", 4)
    pctest(["?#?#?#?#?#?#?#?"], "1,3,1,6", 1)
    pctest(["????", "#", "#"], "4,1,1", 1)
    pctest(["????", "######", "#####"], "1,6,5", 4)
    pctest(["?###????????"], "3,2,1", 10)
    pctest(["?", "???#??##"], "1,10,2", 0)
}

function pctest(records, lengthsStr, want) {
    let got = possibleCovers(records, lengthsStr)
    if (got !== want) {
        console.log("PC TEST FAILURE...", records, lengthsStr, "want", want, "got", got)
    }
}

function possibleRemovals(record, lengthsStr) {
    let memokey = record+";"+lengthsStr
    if (memokey in prmemo) {
        return prmemo[memokey]
    }

    let ans = possibleRemovalsREAL(record, lengthsStr)
    prmemo[memokey] = ans
    return ans
}

// returns map[removedStr]duplicity, null if impossible, removedStr can be ""
function possibleRemovalsREAL(record, lengthsStr) {
    dpr("enter", record, lengthsStr)
    let ans = {}

    if (record === "") {
        if (lengthsStr === "") {
            dpr("return (0)", ans)
            return ans
        } else {
            ans[""] = 1
            dpr("return (1)", ans)
            return ans    
        }
    }

    if (lengthsStr === "") {
        if (record.includes("#")) { // not optional
            dpr("return (2)", "null")
            return null
        } else { // fully optional
            ans[""] = 1
            dpr("return (3)", ans)
            return ans
        }
    }

    let lengths = lengthsStr.split(",").map((n) => Number(n))

    if (!record.includes("?")) { // fully required
        if (lengths[0] === record.length) {
            ans[lengths[0]] = 1
            dpr("return (4)", ans)
            return ans
        } else {
            dpr("return (5)", "null")
            return null
        }
    }

    if (lengths[0] > record.length && record.includes("#")) {
        dpr("return (6)", "null")
        return null
    }

    if (!record.includes("#")) {
        dpr("adding remove-nothing base case")
        ans[""] = 1
    }

    for (let start = 0; start+lengths[0] <= record.length; start++) {
        let offsetRecord = record.slice(start) // bottom of loop body ensures this is "legal"
        let remainderLengthsStr = lengths.slice(1).join(",")
        dpr("loop", start, offsetRecord, ans)

        if (offsetRecord[lengths[0]] !== "#") { // can do the split
            let remainderRecord = offsetRecord.slice(lengths[0]+1)
            let furtherRemovals = possibleRemovals(remainderRecord, remainderLengthsStr)
            dpr("loop furtherRemovals", furtherRemovals)

            if (furtherRemovals) {
                Object.keys(furtherRemovals).forEach((remove) => {
                    if (remove !== "") {
                        let duplicity = furtherRemovals[remove]
                        dpr("loop adding", remove, duplicity)
                        let key = remove ? lengths[0] + "," + remove : lengths[0]
                        ans[key] = (ans[key] || 0) + duplicity    
                    }
                })
            }

            if (!remainderRecord.includes("#")) {
                dpr("loop adding remove-nothing case")
                ans[lengths[0]] = (ans[lengths[0]] || 0) + 1
            }
        }

        if (offsetRecord[0] !== "?") {
            // can't offset the window any further right, so end the loop "early"
            break
        }
    }

    dpr("return (z)", ans)
    return ans
}

function dpr(...args) {
    // console.log("possibleRemovals", ...args)
}

let doprtest = true
if (doprtest) {
    prtest("?#???", "2,1", {"2,1": 3, "2": 2})
    prtest("???", "1", {"1": 3, "": 1})
    prtest("?????", "2,2", {"": 1, "2": 4, "2,2": 1})
    prtest("?###????????", "3,2,1", { '3': 1, '3,2': 6, '3,2,1': 10 })
    prtest("?#?#?#?#?#?#?#?", "1,3,1,6", { '1,3,1,6': 1 })
    prtest("?##??#?#????", "3,3,1", { '3,3': 2, '3,3,1': 6 })
    prtest("??????????", "2,3,2", { '2': 9, '': 1, '2,3': 15, '2,3,2': 4 })
    prtest("###", "3", { '3': 1 })
    prtest("???#??##", "10,2", null)
}

function prtest(record, groupStr, want) {
    let got = possibleRemovals(record, groupStr)
    if (!mediumequal(want, got)) {
        console.log("PR TEST FAILURE...", record, groupStr, "want", want, "got", got)
    }
}

function mediumequal(obj1, obj2) {
    if (!obj1 && !obj2) {
        return true
    }

    if (Object.keys(obj1).length !== Object.keys(obj2).length) {
        return false
    }

    for (let k in obj1) {
        if (obj1[k] !== obj2[k]) {
            return false
        }
    }

    return true
}
