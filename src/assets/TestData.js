//import Node from '@/lib/Node'

const tmplt = { "agencycode":-1,"agencyname":"A","bureaucode":-1,"bureauname":"B","accountcode":-1,"accountname":"C","treasuryagencycode":null,"subfunctioncode":-1,"subfunctiontitle":"subfunc","beacat":"D","onoffbudget":"On-budget","y1976":0,"tq":623,"y1977":0,"y1978":0,"y1979":0,"y1980":0,"y1981":0,"y1982":0,"y1983":0,"y1984":0,"y1985":0,"y1986":0,"y1987":0,"y1988":0,"y1989":0,"y1990":0,"y1991":0,"y1992":0,"y1993":0,"y1994":0,"y1995":0,"y1996":0,"y1997":0,"y1998":0,"y1999":0,"y2000":0,"y2001":0,"y2002":0,"y2003":0,"y2004":0,"y2005":0,"y2006":0,"y2007":0,"y2008":0,"y2009":0,"y2010":0,"y2011":0,"y2012":0,"y2013":0,"y2014":0,"y2015":0,"y2016":0,"y2017":0,"y2018":0,"y2019":0,"y2020":0,"y2021":0,"y2022":0,"y2023":0,"y2024":0,"y2025":0}

export default class TestData {
  rawTree = {top: null, tree: null, total: 0}

  genData (asz = 4, bsz = 4, csz = 4) {
    return this.genraw(asz, bsz, csz)
  }
 
  genraw(asz, bsz, csz) {
    let data = []

    for (let a=0;a<asz;a++) {
      for (let b=0;b<bsz;b++) {
        for (let c=0;c<asz;c++) {
          let acct = Object.assign({}, tmplt)
          acct['agencycode'] = a+1
          acct['agencyname'] = 'A' + (a+1).toString()
          acct['bureaucode'] = b+1
          acct['bureauname'] = 'B' + (b+1).toString()
          acct['accountcode'] = c+1
          acct['accountname'] = 'C' + (c+1).toString()
          acct['y2019'] = a * 20 + b * 10 + c +1
          data.push(acct)
        }
      }
    }
    return data
  }
/*
  genvar (asz, bsz, csz) {
    let data = []
    let idx = 0
    let top = new Node('Total', -1, null)
    let total = 0

    let tsum = 0
    for (let a = asz; a > 0; a--) {
      let anam = 'A' + a.toString()
      let anode = new Node(anam, -1, top)
      top.children.push(anode)
      let asum = 0
      for (let b = bsz; b > 0; b--) {
        let bnam = 'B' + anam + b.toString()
        let bnode = new Node(bnam, -1, anode)
        anode.children.push(bnode)
        let bsum = 0
        for (let c = csz; c > 0; c--) {
          let cnam = 'C' + bnam + c.toString()
          let cnode = new Node(cnam, idx, bnode)
          cnode.sum = a * b * c * 1000
          bsum += cnode.sum
          cnode._idx = idx++
          bnode.children.push(cnode)
          data.push(cnode)
          total += cnode.sum
        }
        bnode.sum = bsum
        asum += bsum
        bnode.children.forEach((itm, idx) => {
          itm.lockVal = itm.sum / bsum
        })
      }
      anode.sum = asum
      tsum += asum
      anode.children.forEach((itm, idx) => {
        itm.lockVal = itm.sum / asum
      })
    }
    top.sum = tsum
    top.children.forEach((itm, idx) => {
      itm.lockVal = itm.sum / tsum
    })
    console.log('sunCHK', tsum, total)
    top.total = total
    this.rawTree.top = top
    this.rawTree.tree = top.children
    this.rawTree.total = total
    return {
      top: top
    }
  }

  gen4 () {
    let data = []
    let idx = 0
    let top = new Node('Total', -1, null)
    top.sum = 10000000
    let tree = top.children
    let total = 0 //  = 1000000
    for (let a = 4; a > 0; a--) {
      let anam = 'A' + a.toString()
      let anode = new Node(anam, -1, top)
      anode.sum = a * 100000
      anode.lockVal = anode.value / top.value
      tree.push(anode)
      for (let b = 4; b > 0; b--) {
        let bnam = 'B' + anam + b.toString()
        let bnode = new Node(bnam, -1, anode)
        bnode.sum = a * b * 10000
        bnode.lockVal = bnode.value / anode.value
        anode.children.push(bnode)
        for (let c = 4; c > 0; c--) {
          let cnam = 'C' + bnam + c.toString()
          let cnode = new Node(cnam, idx, bnode)
          cnode.sum = a * b * c * 1000
          cnode.lockVal = cnode.value / bnode.value
          cnode._idx = idx
          bnode.children.push(cnode)
          data.push(cnode)
          idx += 1
          total += cnode.sum
          console.log(cnam, anode.sum, bnode.sum, cnode.sum)
        }
      }
    }
    console.log(total)
//    this.setNodes(data)
    top.total = total
    this.rawTree.top = top
    this.rawTree.tree = top.children
    this.rawTree.total = total
    return {
      top: top
    }
  }*/
}
