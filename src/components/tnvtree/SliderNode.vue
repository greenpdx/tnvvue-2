<template>
  <div class="slider-node">
    <span class="slider-val"> {{ node.showVal }}</span>
    <div class="slider-input"
      @wheel="onWheel($event)">
      <input
        type="range"
        v-bind:min="min"
        v-bind:max="max"
        v-bind:value="value"
        v-on:input="onChg($event)"
        v-bind:disabled="locked"><br>
      <span class="slider-mm" style="float: left">{{ min }} </span>
      <span class="slider-mm" style="float: right">{{ max }} </span>
    </div>
    <input type="checkbox" id="locknode" v-model="locked">
    <label for="locknode">{{ lockname }}</label>
    <button @click="zeroVal">0</button>
    <button @click="resetVal">Reset</button>
  </div>
</template>

<script>
import { mapGetters } from 'vuex'
import Node from '@/lib/Node'

export default {
  name: 'SliderNode',

  props: {
    node: {
      //type: Node
    }
  },

  data () {
    return {
      min: 0,
      max: 0,
      tmpVal: 0,
      locked: false,
      lockname: 'Lock',
      value: 0
    }
  },

  created () {
    this.defaultVal = this.node.default/1000
    this.value = this.node.val/1000
    this.min = this.defaultVal * 0.5
    this.max = this.defaultVal * 1.5
    console.log("SLD", this.node)
  },

  updated () {
    this.value = this.node.val
  },

  methods: {
    resetVal () {
      this.value = this.node.default/1000
      //this.node.setValue(-1)
    },

    onChg (evt) {
      evt.preventDefault()
      evt.stopImmediatePropagation()
      this.value = evt.target.value
      console.log('inp', evt.target.value)
      //this.node.chgValue(evt.target.value)
    },

    zeroVal () {
      this.value = '0'
      //this.node.setValue(0)
    },

    onWheel (evt) {
      let range = (this.max - this.min) / 30
      let mode = evt.deltaMode
      let step = (evt.deltaY > 0) ? range : -range
      this.value = step + this.value
      console.log(mode, step)
      //this.node.chgValue(this.value)
    }
  },

  watch: {
    node: {
      value: function (val, old) {
        console.log('Slider',
          Object.assign({}, val),
          Object.assign({}, old))
      }
    }
  },

  computed: {
    ...mapGetters({
      total: 'total'
    })
  }
}
</script>

<style scoped>
.slider-node {
  display: block;
  background-color: #ddd;
  z-index: 10;
  opacity: 1;
  padding: 3px;
  border: 2px solid #000;
  position: relative;
}
.slider-val {
  display: inline-block;
  width: 3em;
}
.slider-mm {
  font-size: .5em;
  margin-top: -1.5em;
}
.slider-input {
  display: inline-block;
}
</style>
