<template>
  <div id="tnvapp">
    <tnv-header></tnv-header>
    <div id="main">
      <div id="navtab">
        <router-link class="navbutton" :to="{ name: 'home', params: {} }">Home</router-link>
        <router-link class="navbutton" :to="{ name: 'vote', params: {} }" replace>Vote</router-link>
        <router-link class="navbutton" :to="{ name: 'info', params: {} }" replace>About</router-link>
        <router-link class="navbutton" :to="{ name: 'contact', params: {} }" replace>Contact</router-link>
        <router-link class="navbutton" :to="{ name: 'revolt', params: {} }" replace>Revolt</router-link>
        <!--router-link :to="{ name: 'login', params: {share: share }}" :crypt="share" replace>Login</router-link -->
      </div>
      <section style="margin-bottom: 2em;">
        <router-view></router-view>
      </section>
    </div>
    <div style="margin-top: 1em;">
      <tnv-footer></tnv-footer>
    </div>
  </div>
</template>

<script>
import axios from 'axios'
import TestData from '@/assets/TestData'


//import nacl from 'tweetnacl'
//import base64 from 'base64-js'
//import Rpc from '@/components/Rpc'

import TnvFooter from '@/components/TnvFooter'
import TnvHeader from '@/components/TnvHeader'
/*
function rdSession () {
  let pkey = sessionStorage.getItem('pkey')
  let skey = sessionStorage.getItem('skey')
  let ts = sessionStorage.getItem('ts')

  if (ts && ts + 10 > Date.now()) {
    skey = null
    console.log('new')
  }

  let key = {sess: '', key: ''}
  if (!skey) {
    key = nacl.box.keyPair()
    console.log(key.secretKey.length)
    wrSession({key: key})
  } else {
    console.log(pkey, skey)
    key.sess = base64.toByteArray(pkey)
    key.key = base64.toByteArray(skey)
  }
  return key
}
function wrSession (conf) {
  let key = conf.key
  let pkey = base64.fromByteArray(key.publicKey)
  let skey = base64.fromByteArray(key.secretKey)
  sessionStorage.setItem('pkey', pkey)
  sessionStorage.setItem('skey', skey)
  sessionStorage.setItem('ts', Date.now())
} */
/*
const share = {
  key: '',
  sess: '',
  nonce: '',
  salt: '',
  pkey: '',
  pnonce: '',
  email: ''
}
*/
export default {
  name: 'tnvapp',
  mixins: [
  //  Rpc
  ],
  components: {
    'tnv-header': TnvHeader,
    'tnv-footer': TnvFooter
  },
  data () {
    return {
      data: null,
      share: "",
      test: false
    }
  },
  beforeCreate () {
  },
  created () {
    this.getData()
    //let conf = this.rdSession()
    //this.share.key = conf.secretKey
    //this.share.sess = conf.publicKey
    //this.share.nonce = nacl.randomBytes(nacl.box.nonceLength)
    //let wasm = this.$wasm
    //let rtn = wasm.init_app([])
    //console.log('APP>', rtn)
 /*   this.sendRpc('hello', {
      hello: base64.fromByteArray(this.share.nonce)
    }, (rslt, error) => {
      if (!error) { // rslt is just json data
        if (rslt.hello.length === 44) {
          this.share.pkey = base64.toByteArray(rslt.hello)
          this.share.fast = nacl.box.before(this.share.pkey, this.share.key)
          console.log('APP<', rslt, this.share.pkey, this.share.fast)
        } else {
          alert('Stop Hacking')
          window.location.assign('/hacking?trys=many')
        }
      } else { // the rslt is the response
        console.log('HELLO!', rslt)
      }
    })*/
  },
  methods: {

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

    getData () {
      let self = this
      let wasm = self.$wasm
      wasm.greet("O TEST")
      let data = []
      if (this.test) {
        let td = new TestData()
        let data = td.genData(2,2,2)

        let rawdata = data.sort(self.nodeSort)
        self.$root.rawdata = rawdata
        console.log( typeof rawdata)
        let rtn = wasm.raw2accts(rawdata)
        wasm.greet("T TEST")
        console.log("DATA", rtn, rawdata)
      } else {
//      axios.get('http://localhost:8181/budget/full/_find?batch_size=5000')
        axios.get('/mongodb')
//      axios.get('http://10.0.42.104:8181/mongodb')
//      axios.get(process.dbURL)
//      axios.get('http://10.0.42.126/full.json')

          .then(response => {
            //console.log(response)
            let rslt = response.data
            let w = self.$wasm
            console.log(wasm, w)
            wasm.greet("I TEST")
            rslt.sort(self.nodeSort)
            let rtn = wasm.raw2accts(rslt)
            console.log(rtn)
            let tre = wasm.gen_tree(rtn)
            self.$root.rawdata = rtn 
            console.log(tre)             
          })
          .catch(error => {
            console.log(error)
          })
      }
    }
  }

}
</script>

<style>
#tnvapp {
  display: block;
  font-family: 'Avenir', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 10px;
}
#navtab {
  font-size: 2em;
  text-align: left;
}
#main {
  display: inline-block;
  width: 100%;
  height: 100%;
}
.navbutton {
  padding: 11px;
}
</style>
