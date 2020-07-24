<template>
  <div class="tree-view-node">
    <div class="tvn-node">
        <div class="tvn-line" @click="selClick">
          <span class="tvn-amount"> {{ node.showVal }}</span>
          <span class="tvn-name"> {{ name }} </span>
        </div>
        <slider-node
          v-if="node.select"
          :node="node"></slider-node>
      </div>
  </div>
</template>

<script>
import { mapGetters, mapActions } from 'vuex'
import Node from '@/lib/Node'

import SliderNode from './SliderNode'

export default {
  name: 'TreeViewNode',

  components: {
    SliderNode
  },

  props: {
    nodeIdx: Number,
    level: {
      default: 0
    }
  },

  data () {
    return {
      node: null,
      total: 0,
      locked: false,
      change: false
    }
  },

  created () {
    //console.log(this.nodeIdx, this.$root.tree)
    this.node = Object.assign({}, this.$root.tree[this.nodeIdx])
    this.node['select'] = false
    this.node['expand'] = true
    this.node['hover'] = false
    //this.node['showVal'] = this.showVal

    //this.node['showVal'] = this.showVal
    //this.total = this.node.total
    //this.children = this.node.chld
    //console.log(this.node)
  },

  mounted () {
    this.change = true
  },
  updated () {
    //this.dbgPrt('tnodeUpdate')
  },

  methods: {
    ...mapActions([
      'setSelect',
      'setHover',
      'setExpand'
    ]),
    showVal() {
      let sum = 0
      let chld =  this.node.chld
      if (chld.length > 0) {
        for (c in chld) {
          sum += this.showVal
        }
      } else 
        return this.node.val
      return sum
    },
    selClick () {
      if (this.node.select) {
        this.node.select = false
        //this.setSelect(node)
      } else {
        this.node.select = true
        //this.setSelect(node)
      }
      this.$forceUpdate()
      console.log("SEL",this.selected, this.node.select)
    },
  },

  computed: {
    ...mapGetters([
      'rawData',
      'getNodeByIdx'
    ]),
    expanded: function () {
      return this.node.expand
    },
    selected () {
      return this.node.select
    },
    name: function () {
      return this.node.name
    },
    hovered: function () {
      return this.node.hover
    },
    //nodes: function () {
    //  return this.children
    //},
    indent: function () {
      let lvl = 'level0'
      switch (this.level) {
        case 2:
          lvl = 'level2'
          break
        case 1:
          lvl = 'level1'
          break
        default:
      }
      return lvl
    }
  }
}
</script>

<style scoped>
.tree-view-node {
  display: inline-block;
  float: left;
  width: 45em;
}
.tvn-expand {
  display: inline;
}
.noexpand {
  float: left;
  text-align: right;
  width: 3em;
  cursor: cell;
}
.tvn-expander {
  float: left;
  text-align: left;
  cursor: pointer;
}
.level0 {
  float: left;
  text-align: left;
  cursor: pointer;
  width: 3em;
}
.level1 {
  float: left;
  text-align: left;
  cursor: pointer;
  width: 2em;
  padding-left: 1em;
}
.level2 {
  float: left;
  text-align: left;
  cursor: pointer;
  width: 1em;
  padding-left: 2em;
}
.tvn-node {
  display: inline-block;
  float: left;
  width: 100%
}
.tvn-line {
  text-align: left;
  cursor: cell;
}
.tvn-amount {
  display: inline-block;
  width: 4em;
  text-align: right;
  margin-right: 1em;
}
.tvn-name {
}
</style>
