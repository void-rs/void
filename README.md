# void

> The primary thing when you take a sword in your 
hands is your intention to cut the enemy, whatever 
the means. Whenever you parry, hit, spring, strike 
or touch the enemy’s cutting sword, you must cut 
the enemy in the same movement. It is essential to 
attain this. If you think only of hitting, springing, 
striking or touching the enemy, you will not be able 
actually to cut him. More than anything, you must 
be thinking of carrying your movement through to 
cutting him. ...When you appreciate the power of 
nature, knowing the rhythm of any situation, you 
will be able to hit the enemy naturally and strike 
naturally. All this is the Way of the Void.
- Miyamoto Musashi

![](/demo.gif)

#### install

`cargo install voidmap`

if you don't have cargo, an easy way to get and manage 
it is [via rustup](https://www.rustup.rs/).

#### invocation

`void [/path/to/savefile]`

#### keys

feature | control 
--- | ---
new node (freeform) | click blank space 
new node (child of selected) | tab
new node (sibling of selected) | ^t
delete selected node and its children | Delete
mark selected node complete | ^a
drill-down into selected node | ^w
pop up selection | ^q
hide children of selected | Enter
open text editor for `txt:...` node | ^v
prefix-jump with no selection | type a letter
prefix-jump with other selected | ^f
hide completed children of node | ^h
select arrow start/destination | ^r
erase arrow | select start, ^r, then destination, ^r
show debug log | ^l
reparent node | drag node to new parent
move subtree | drag parent to new location
auto-arrange nodes in view | ^p
scroll up | PgUp
scroll down | PgDn
select up | Up
select down | Down
select subtree to left | Left
select subtree to right | Right
de-select node | Esc
save | ^x
exit | Esc with nothing selected
exit | ^c

#### known bugs

doesn't properly handle very long text. if you want to embed 
an essay, create a node that begins with `txt: ` and hit `^v` 
to open its contents.

#### optional configuration

setting the `LOGFILE` environment variable will allow you to
log debugging info to a file.

setting the `EDITOR` environment variable will allow you to
specify which text editor is opened when hitting `^v` on a 
node whose name begins with `txt: `.  defaults to vim.

#### notes

This came about in the midst of an (ongoing) obsessive inquiry into a
cluster of topics roughly related to "effectiveness" while stumbling
through various mountain ranges and cities in central europe and the
american northeast.

* conversations with [@matthiasn](https://github.com/matthiasn) and being introduced
to his wonderful [iWasWhere](https://github.com/matthiasn/iWasWhere) system
* [writings of eliezer s. yudkowsky](https://wiki.lesswrong.com/wiki/Rationality:_From_AI_to_Zombies),
thinking fast and slow, the optimism bias, [how to solve it](https://en.wikipedia.org/wiki/How_to_Solve_It)
* [various subtopics of operations research](https://en.wikipedia.org/wiki/Operations_research#Problems_addressed)
* occult mindfulness: [undoing yourself with energized meditation]
(http://heruka.altervista.org/X_files/Undoing%20Yourself%20With%20Energized%20Meditation%20And%20Other%20Devices%20-%20Christopher%20S%20Hyatt.pdf),
[prometheus rising](http://www.principiadiscordia.com/downloads/04%20Prometheus%20Rising.pdf)
* military C2 theory, recognition/metacognition, OODA, etc... [A Review of Time Critical Decision Making Models and 
Human Cognitive Processes](https://pdfs.semanticscholar.org/2eb9/e12955dfafd4ab5d9337b416e31f5afca834.pdf)
* personal productivity literature: [pomodoro](http://baomee.info/pdf/technique/1.pdf), [GTD](https://en.wikipedia.org/wiki/Getting_Things_Done),
[eat that frog](http://www.actnow.ie/files/BookSummaryEatThatFrog.pdf), [flow](http://216.119.127.164/edgeware/archive/think/main_filing15.htm)

