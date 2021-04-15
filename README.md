# vimers
A Telegram bot that fetches tips and tweets from @vim_tricks. It is possible to visualize the data from terminal as well if preferred.  

## Installation

To run it clone the repository 

```sh
git clone https://github.com/bluretrece/vimers.git
cd vimers/
```
## Running
Make sure you have a ```consumer_key``` and a ```consumer_secret``` before you try to compile vimir. Running ```cargo run```, should be enough to make everything work out of the box.

```sh
cargo run
```
## Examples

```
VimTricks (@vim_tricks) posted at 2021-04-12 14:49:49 -04:00
Here's a useful way to get a better view of the code surrounding your current line:

• zt - reposition viewport so your current line is at the top
• zz - middle
• zb - bottom

```
```
VimTricks (@vim_tricks) posted at 2021-04-14 20:58:54 -04:00
Resize your splits in Vim:

• ctrl-w - – decrease split height by 1 line
• ctrl-w + – increase
• ctrl-w &gt; – increase split width by 1 line
• ctrl-w &lt; – decrease
```
## To do
- [ ] Fetch tweets without the need of key credentials (if possible).
- [ ] Pretty-print the output and better formatting of it.
