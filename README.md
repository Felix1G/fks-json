# fks-json
 JSON Parser using Rust I made for fun. Includes a lot of error handling.

A piece of JSON code valid according to my parser:

```javascript
{
 "name": "Felix",
 "nickname": null, //Just call me Felix actually :/
 "age": 16,
 "boy": true,
 "numberX": 0x123,
 "numberO": 0o7712,
 "numberB": -0b1101101,
 "numberE": 1.3445e245,
 "numberf": 1.3234f,
 "numberF": -0.000023F,
 "numberd": -0.0123123d,
 "numberD": 10001000100D,
 "float": 0.00001,

 /*
 Time for arrays!
 */
 "items": [
  {
   "name": "Axe",
   "damage": 1e-10,
  },

  {
   "name": "Sword",
   "damage": 100,
  },

  {
   "name": "Mace",
   "damage": 1e+100,
  }, //accepts trailing commas
 ]
}
```
