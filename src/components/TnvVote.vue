<template>
  <div class="tnvvote">
    <div>
      <span>Change your Tax Dollar </span>
      <button @click="btnLoad('l')">Load</button>
      <input type="text" v-model="in0"/>
      <button @click="btnSave('s')">Save</button>
    </div>
   <!-- TEST -->
    <!-- div>
      <div v-for="(acct, idx) in tst" :key="idx">
        {{ acct.agencyname }} {{  acct.bureauname }} {{ acct.accountname }}
      </div>
    </div -->
    <!-- div class="split">
      <div v-if="webglEnabled" id="three3d" class="three3d" ref="three3d">
        <div>
          <span style="font-weight: bold;">{{ showWhat }}</span>
          <div class="help3d">
            <div
              @click="popHelp"
              class="helptext">
              <span>{{ helpLabel }}</span>
            </div>
            <div v-if="showHelp" id="popHelp" @click="popHelp">
              <div v-for="(line, idx ) in helpLines" :key="idx">
                <span>
                  {{ line }}
                </span><br/>
              </div>
            </div>
          </div>
        </div>
        <div class="render-area">
          <span class="wait">{{ waitmsg }}</span><br>
          <span style="font-size: 2em;"> {{ showWhat }} </span>
        <tnv-3d v-if="top"
          :top="top"
          :size="threeSize"
          :camPos="camPos"
          :freeze="freeze"
          id="tnv3d">
        </tnv-3d >
        <div v-show="showInfo" class="infopop" ref="infopop">
          <span>{{ info.name }}</span><br>
          <span>{{ info.value / 1000 }} Million</span><br>
          <span>{{ info.percent }}</span> % of {{ info.parent }} Budget
        </div>
      </div>

      <div class="controls">
        <fieldset class="chkgroup">
          <legend>Select display category</legend>
          <div class="chklist">
            <label class="chkinput"><input v-model="beacat" type="checkbox" value="discretionary"/>
              <span class="chkbox">Discretionary</span></label>
            <label class="chkinput"><input v-model="beacat" type="checkbox" value="mandatory"/>
              <span class="chkbox">Mandatory</span></label>
            <label class="chkinput"><input v-model="beacat" type="checkbox" value="interest">
              <span class="chkbox">Interest</span></label>
          </div>
        </fieldset>
        <div class="chklist">
          <select @change="onSelect($event)" class="chkinput" v-model="year">
            <option class="chkinput" v-for="(n, idx) in range(1976,2017)" :key="idx">{{ n }}</option>
          </select>
        </div>
        <button @click="onClick">Test</button>
      </div>
    </div -->
    <div>
      <drop 
        @dragenter="de"
        @dragleave="de"
        @dragover="de"
        @drop="drp">
        <div id="querybox">
          <span>Compare Accounts</span>
          <div v-for="node in cmpnodes" :key="node.idx">
            {{ node.val }} {{ node.name }}
          </div>
        </div>
        </drop>
      <div class="tnv-tree">
        <tree-view v-if="isLoaded">
        </tree-view>
      </div>
    </div>
    <div class="render-area">
          <!--span class="wait">{{ waitmsg }}</span><br>
          <span style="font-size: 2em;"> {{ showWhat }} </span -->
      <tnv-3d v-if="top"
          :top="top"
          :size="threeSize"
          :camPos="camPos"
          :freeze="freeze"
          id="tnv3d">
      </tnv-3d>
    </div>
   </div>

<!-- /div -->
</template>

<script>
import { mapGetters, mapActions } from 'vuex'
import * as THREE from 'three'
import Rpc from './Rpc'
import axios from 'axios'
import Node from '@/lib/Node'
//import Drop from "../../lib/src/components/Drop.vue";
import { Drag, Drop, DropMask } from "vue-easy-dnd"

import Tnv3D from './tnv3d/Tnv3D'
import TreeView from './tnvtree/TreeView'

export default {
  name: 'TnvVote',
  mixins: [
    Rpc,
    Drop
  ],
  components: {
    'tnv3d': Tnv3D,
    'tree-view': TreeView
  },

  data () {
    return {
      cmpnodes: [],
      in0: 0,
      objInfo: '',
      waitmsg: "Under construction",
      threeSize: {
        x: '800',
        y: '800'
      },
      helpLabel: '3D Navigation Help',
      showHelp: false,
      showWhat: 'US government 2016 discretionary budget',
      helpLines: [
        '> Move MouseDown to rotate view',
        '> ScrollWheel to zoom',
        '> Mouse Hover displays information about agency, bureau,or department',
        '> Right click on hex to select / unselect the agency, bureau or dept',
        '> Adjust value of selected object by ScrollWheel',
        '> Double click on hex to zoom to next level',
        '> Double click on outside hex to unzoom'
      ],
      grppos: {
        'x': 0,
        'y': 10,
        'z': 0
      },
      //beacat: ['discretionary'],
      beacat: ['d'],
      year: 2016,
      selectedYear: 'y2019',
      data: {},
      nodes: [],
      size: {
        x: 800,
        y: 800
      },
      camPos: {
        x: -50,
        y: 150,
        z: -100
      },
      freeze: {},
      //top: null,
      test: true,
      rawTree: {},
      info: {
        name: '',
        value: 0,
        percent: 0,
        parent: ''
      }
    }
  },

  beforeCreate () {
    if (!window.WebGLRenderingContext) {
      this.noWebgl()
    }
  },

  created () {

//    let rand = Math.floor(Math.random() * (Number.MAX_SAFE_INTEGER - 1))
  },

  mounted () {
    let wasm = this.$wasm
    let srv = this.top
    //let bdgt = wasm.init_app(this.tst)
    console.log(srv)
  },

  watch: {

//    top is the start of the tree
//    expandNode: function (node) {
//    }
  },
  computed: {
    ...mapGetters({
      hoverNode: 'hoverNode',
      selectNode: 'selectNode',
      expandNode: 'expandNode',
      webglEnabled: 'webglEnabled',
      getNodes: 'getNodes',
      isLoaded: 'isLoaded',
    }),

    showInfo () {
      if (this.hoverNode !== null) {
        let node = this.hoverNode
        //this.info.name = node.name
        //this.info.value = node.value
        //this.info.percent = Math.round(node.lockVal * 1000000) / 10000
        //this.info.parent = node.parent.name
        return node
      } else {
        //this.objInfo = ''
        return null
      }
      //vecnodes () {
      //  return this.$
      //}
    },

    top() {
      let ns = this.$store.getNodes
      console.log(ns)
      return ns
    }
  },

  methods: {
    ...mapActions([
      'setActive',
      'setExpand',
      'noWebgl'
    ]),
    de (evt) {
      console.log('de',evt)
    },
    drp (evt) {
      let node = evt.data
      this.cmpnodes.push(node)
      console.log('drp',node.name)
    },
    async btnLoad (ls) {
      console.log('LOAD')
      let wasm = this.$wasm
      wasm.load_template('/budauth_min.csv')
        .then((csv) => {
          console.log(csv) 
        })
        .catch((err) => { console.log(err) })
    },
    async btnSave() {
      let obj = this.getNodes
      let ary = []
      //let k = obj.keys()
      let cnt = 0;
      for (let v in obj) {
        let itm = obj[v]
        cnt = cnt + 1;
        if ( itm.key.acode < 0 || itm.key.bcode < 0 || itm.key.ccode < 0 ) continue
        if ( itm.val == 0 ) continue
        //if ( itm)
        let t = { acode: itm.key.acode, bcode: itm.key.bcode, ccode: itm.key.ccode, val: itm.val}
        ary.push(t)
      }
      console.log('SAVE',cnt, ary.length)
        

    },

    sortSum (a, b) {
      if (a.sum > b.sum) { return -1 }
      if (a.sum < b.sum) { return 1 }
      return 0
    },
    setLockVal (itm, idx) {
      itm.lockVal = itm.value / itm.parent.default
//      console.log(itm.value, itm.parent.default)
    },

    nodeSort(n1, n2) {
      if (n1.agencycode == n2.agencycode) {
        if (n1.bureaucode == n2.bureaucode) {
          if (n1.accountcode == n2.accountcode) {
            return n1.value - n2.value
          } else {
            return n1.accountcode - n2.accountcode            
          }
        } else {
          return n1.bureaucode - n2.bureaucode 
        }
      } else {
        return n1.agencycode - n2.agencycode
      } 
    },

    groupData (budgt, filterCB, year) {
      /*let agcy = {}
      let bgdt = {}
      let acct = {}
      let total = 0
//      Node.clrNodes()
      let tree = []
      let top = 0
      tree.push(new Node('Total', 0, -1))

      budgt.forEach((itm, idx) => {
        if (!filterCB(itm)) {
          return
        }
        let idx = tree.length
        if (itm.agencycode in agcy) {

        } else {
          let an = new Node(itm.agencyname, idx, ) 
          agcy['itm.agencycode'] = 
        }
        let node = new Node(itm.agencyname, idx, parent)
        tree.push(node)
        let val = itm[year]
        total += val
        //let parent = 
        /*
        if (!map[itm.agencycode]) {
          map[itm.agencycode] = new Node(itm.agencyname, idx, parent)
          tree.push(map[itm.agencycode])
        }
        parent = map[itm.agencycode]
        map[itm.agencycode].sum += val
        if (!map[itm.agencycode].children[itm.bureaucode]) {
          map[itm.agencycode].children[itm.bureaucode] =
          new Node(itm.bureauname, idx, parent)
        }
        parent = map[itm.agencycode].children[itm.bureaucode]
        map[itm.agencycode].children[itm.bureaucode].sum += val
        let tmp = new Node(itm.acctname, idx, parent)
        tmp.sum = val
        map[itm.agencycode].children[itm.bureaucode].children.push(tmp)
      })*/
/*
      top.total = total
      top.value = total
      top.default = total
      tree.sort((a, b) => this.sortSum(a, b))
      tree.forEach((itm, idx) => this.setLockVal(itm, idx))
      for (let a of tree) {
        let achld = Object.values(a.children)
        achld.sort((a, b) => this.sortSum(a, b))
        achld.forEach((itm, idx) => this.setLockVal(itm, idx))
        a.children = achld
        for (let b of achld) {
          let bchld = b.children
          bchld.sort((a, b) => this.sortSum(a, b))
          bchld.forEach((itm, idx) => this.setLockVal(itm, idx))
          b.children = bchld
        }
      }
      this.rawTree.total = total
      this.rawTree.tree = tree
      this.rawTree.top = top
      return {total: total, tree: top}*/
    },

    haveData (data, self) {
      this.data = data
      return
      /*if (this.test) {
        this.top = this.data
        this.setExpand(this.top)
      } else {
        let rslt = this.groupData(data, this.filterData, 'y2019')
        console.log(rslt.tree.children.length)
        this.top = rslt.tree
      }*/
    },
    range (min, max) {
      let ary = []
      for (let i = max; i >= min; i--) {
        ary.push(i)
      }
      return ary
    },
    onClick (evt) {
    },
    popHelp (evt) {
      this.showHelp = !this.showHelp
    },
    filterData (itm, idx) {
      let beacat = itm.beacat[0].toLowerCase()
      let set = new Set(this.beacat)

      if (!set.has(beacat)) {
        return false
      }
      if (itm.onoffbudget !== 'On-budget') {
        return false
      }
      if (itm[this.selectedYear] === 0) {
        return false
      }
      if (itm[this.selectedYear] < 0) {
        return false
      }
      itm['value'] = itm[this.selectedYear]
      this.total += itm[this.selectedYear]
      return true
    },

    loadClick (evt) { // try bezier  gre/bezier-easing
      let camera = this.$refs.camera0
      let sel = this.selectObj
      let old = camera.curObj.position
      console.log('CAM', old)
      if (sel !== null) {
        console.log(sel.loc) // just change

        /*        let dif.x = (old.x - sel.loc.x) / 100
              let dif.y = (old.y - sel.loc.y) / 100
              let dif.z = (old.z -sel.loc.z) / 100
              let cnt =
              let id = setInterval(function () {
                if (cnt-- < 0) {

                }
                camera.curObj.position.set(sel.loc.x, sel.loc.y + 50, sel.loc.z)
                camera.curObj.lookAt(new THREE.Vector3(sel.loc.x, sel.loc.y, sel.loc.z))
              }, 10) */
      }
    },

    clrClick (evt) {
      this.clearNodes()
    },
    clickIt (evt) {
      console.log(evt)
    },
    zoomIn (node) {

    },
    dumpClick (evt) {
      let nodes = this.getNodes
      nodes.forEach((itm, idx) => {
        let val = this.getNodeById(itm._id)
        val.then(node => console.log(itm._id, node._id))
      })
      let itm = this.getNodeById(this.tstId)
      itm.then(node => console.log(this.tstId, node._id))
    },
    getData () {
      let self = this
//      axios.get('http://localhost:8181/budget/full/_find?batch_size=5000')
      axios.get('/mongodb')
//      axios.get('http://10.0.42.104:8181/mongodb')
//      axios.get(process.dbURL)
//      axios.get('http://10.0.42.126/full.json')

        .then(response => {
          console.log(response)
          let rslt = response.data
          
          self.data = rslt.sort(self.nodeSort)
          //self.haveData(self.data, self)
//          self.setNodes(data)
        })
        .catch(error => {
          console.log(error)
        })
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
#querybox {
  width: 40%;
  height: 30em;
  display: block;
  float: left;
  left: 0;
  top: 0;
  border: 0.1em solid #000;
  text-align: left;
}
.tnvvote {
  display: inline-block;
}
.wait {
  font-size: 4em;
  color: #080;
  text-decoration: blink;
  font-weight: bold;
  animation: blinker 1s ease-out 5;
}
@keyframes blinker {
  50% { opacity: 0; }
}
.split {
  display: flex;
}
.three3d {
  display: block;
  float: left;
}

.render-area {
  display: block;
  position: relative;
  height: 100px;
  width: 100px;
  border: 3px solid #000;
}
.infopop {
  border: 2 solid black;
  width: 30em;
  height: 100px;
  position: absolute;
  display: inline-block;
  top: 0px;
  left: 0px;
  background-color: yellow;
  z-index: 10;
}

.tnv-tree {
  display: inline-block;
  margin-left: 1em;
  width: 45%;
  height: 800px;
  text-align: left;
}
.right {
  float: right;
  width: 45%;
  text-align: left;
  font-size: 1.2em;
}

#container {
  position: relative;
}

#renderer {
  position: absolute;
  top: 0;
  left: 0;
  margin-top: 40;
}

#overlay {
  position: absolute;
  z-index: 10;
  top: -20;
  left: 0;
  width: 800px;
  height: 800px;
  pointer-events: none;
}
.help3d {
  display: inline;
  float: right;
  width: 80%;
  margin-left: -50%;
  position: relative;
}
.helptext {
  float: right;
  display: inline-block;
  cursor: pointer;
  border: 1 solid #000;
  background-color: #c88;
}
#popHelp {
  position: absolute;
  display: block;
  right: auto;
  top: auto;
  z-index: 20;
  background-color: #448;
  color: #fff;
  font-size: 1em;
  text-align: left;
  right: 0;
}
h1,
h2 {
  font-weight: normal;
}

ul {
  list-style-type: none;
  padding: 0;
}

li {
  display: inline-block;
  margin: 0 10px;
}

a {
  color: #42b983;
}
</style>
