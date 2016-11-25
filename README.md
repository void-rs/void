# void

## Problems

This is an attempt to address several cognitive defects.

1. frequently fall out of creative flow
1. day-to-day work lacks coherence
1. failure to integrate learnings into a cohesive perspective
1. execution of tasks lacks focus and motivation
1. unclear how my efforts are impacting goals

## Perspectives

* things we measure tend to improve
* we should minimize decisions to prevent fatigue
* we should regularly reevaluate priorities
* flexible cores can be bent to many uses

## Implementation

* everything is a tree
* you can collapse subtrees
* you can drill-down the screen focus arbitrarily
* trees of tasks can be marked with `#task`, all children of marked nodes are implicitly subtasks
* tasks can be prioritized with `#prio=<n>`, all children implicitly inherit the lowest ancestor's priority
* a task can be chosen automatically, with priorities weighting a random selection. you should delete it or do it, don't get into the habit of drawing again until you see something you like.  you chose the priorities, and you should keep them up-to-date.
* you can draw arrows between nodes for mind-mapping functionality

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
-Miyamoto Musashi

![](/demo.gif)

#### install

`cargo install voidmap`

if you don't have cargo, an easy way to get and manage 
it is [via rustup](https://www.rustup.rs/).

#### invocation

`void [/path/to/savefile]`

#### keys

feature | control | feature | control
--- | --- | --- | ---
new node | ^n | new node (child of selected) | Tab
new node (freeform) | click blank space | new node (sibling of selected) | Enter
delete selected node and its children | Delete | move subtree | drag parent to new location
arrange nodes in view | ^p | auto-arrange nodes in view from now on | ^z
mark selected node complete | ^a | drill-down into selected node | ^w
pop up selection | ^q | hide children of selected | ^t
open text editor for `txt:...` node | ^k | prefix-jump with no selection | type a letter
prefix-jump with other selected | ^f | hide completed children of node | ^h
select arrow start/destination | ^r | erase arrow | select start, ^r, then destination, ^r
show debug log | ^l | reparent node | drag node to new parent
scroll up | PgUp | scroll down | PgDn
select up | Up | select down | Down
select subtree to left | Left | select subtree to right | Right
de-select node | Esc | save | ^x
exit | Esc with nothing selected | exit | ^c
jump to weighted next task | ^v
#### known bugs

doesn't properly handle very long text. if you want to embed 
an essay, create a node that begins with `txt: ` and hit `^v` 
to open its contents in an external text editor, specifiable
by setting the `EDITOR` env var.

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

