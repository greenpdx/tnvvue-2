<template>
  <div id="tnvhome">
    <div class="tnvwhat">
      <span style="font-size: 1.5em; font-weight: bold;">Vote with your Tax Dollar, Create the People's Budget</span><br>
       <h4>One Person, One Tax Dollar</h4>
       <p>Right now a red person in  blue state and a blue person in a red states vote do not count. 
       Gerrymandering law suites are dividing the country. Tax N Vote every body has one vote.</p>
       <h4>Divide your Tax Dollar to 1/1,000,000,000 of a penny</h4>
      <p>A Tax Dollar is a proportional representation of the U.S. budget. One penny is one percent of 
      the budget. Allocate five cents to Department of Veterans Affairs, that means five percent of
       the US budget goes to the VA.</p>  
      <h4>The People tell Congress what is important to them in Hard Data. Not Fake News, Not Alt-Facts.</h4>
      <p>Tax N Vote provides a numeric objective measure of what people want. People do not trust the news.
        This system creates hard data. Every vote counts. There is a digital and hardcopy protocols
        so every vote can be verified. Blockchain security is used to verify every vote.</p>   


       <p> What is the truth? oday </p>
      <p>Everybody, it does not matter red or blue, agrees that they would like have more control of
       how they want their taxes spent. The president submits a budget to congress, Congress should
        accept The People's Budget. The average everybody's vote and that makes the budget. 
      </p>
      <h4></h4>

      
    </div>
    <div>
      <h4 style="margin-bottom: 0in;">Benefits of Tax N Vote:</h4>
      <ul class="tnvbenefits">
        <li>Apolitical, does not matter republican, democrat or independent</li>
        <li>Congress will really know what the people want</li>
        <li>The result is hard data, no alt-facts or fake news</li>
        <li>Zero cost to person, Tax N Vote is web or mobile based and does not cost to use</li>
        <li>Everybodys ideas and values are counted. No gerrymandering or geographicly biased</li>
        <li>Easy to use, just submit a template or adjust the budget to exactly what is wanted</li>
        <li>Advertising for templates will educate the people about the budget</li>
      <li>People will feel better about paying taxes when the think they have a voice on
        how it is spent</li>
      <li>Tax compliance will increase</li>
      <li>The results can be used to measure politicians in next election</li>
      <li>Does not take a constitutional convention to change the system</li>
      <li>Low overhead cost, the total amount of anonymous data can fit on $400 of disk space</li>
      <li>Individuals can verify their own data using the receipt number</li>
      <li>Hard data, not fake news or alt facts</li>
      </ul>
    </div>
    <div>
      <tnv-flow></tnv-flow>
    </div>
  </div>
</template>

<script>
import TnvFlow from './TnvFlow'

export default {
  name: 'TnvHelp',
  components: {
    'tnv-flow': TnvFlow
  },
  data () {
    return {
      helpwrite: 'Home'
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
#tnvhome {
  display: inline-block;
  height: 100%;
  width: 100%;
  text-align: left;
  margin-top: 2em;
}
.injrev {
  text-align: center;
  font-size: 4em;
  font-family: arial;
  font-weight: bold;
  background-color: yellow;
  text-shadow:
  -2px -2px #000,
  2px -2px #000,
  -2px 2px #000,
  2px 2px #000;
  border: 2px solid #000;
  text-decoration: none;
}
.tnvwhat {
  font-size: 1.4em;
  margin-top: 1em;
}
.tnvbenefits li {
  margin-top: 10px;
}
</style>
