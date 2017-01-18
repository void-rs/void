# void [![Build Status](https://img.shields.io/travis/spacejam/void.svg?style=flat-square)](https://travis-ci.org/spacejam/void) ![State](https://img.shields.io/badge/state-alpha-orange.svg?style=flat-square)

[Tutorial](TUTORIAL.md)

[Example Workflow](#what-i-do-dont-do-what-i-do-discover-what-works-for-you)

WARNING: this is alpha, and the default keybinds are still weird because I use colemak on top of tmux. You may want to change them, by setting the `KEYFILE` env var to the path to a [key remap file](default.keys). In the future, I may add optional modal editing to bring it more in-line with vim. Right now I'm not sure it's worth the extra keystrokes.

Feedback encouraged! If you have a hard time with something, let me know about it, and I'll work to smooth out the experience!

![](/demo.gif)

## problems

This is an attempt to address several cognitive defects.

1. frequently fall out of creative flow
1. day-to-day work lacks coherence
1. failure to integrate learnings into a cohesive perspective
1. execution of tasks lacks focus and motivation
1. unclear how my efforts are impacting goals

## perspectives

* things we measure tend to improve
* we should regularly reevaluate priorities
* we should minimize decisions to prevent fatigue
* individual sensemaking is well served by reflection, journaling, outlining, mind-mapping, etc...
* don't impose specific workflows, but support many possibilities

## implementation

* everything is a tree
* you can collapse subtrees
* you can drill-down the screen focus arbitrarily
* trees of tasks can be marked with `#task`, all children of marked nodes are implicitly subtasks
* tasks can be prioritized with `#prio=<n>`, all children implicitly inherit the lowest ancestor's priority
* a task can be chosen automatically, with priorities weighting a random selection. you should delete it or do it, don't get into the habit of drawing again until you see something you like.  you chose the priorities, and you should keep them up-to-date.
* you can create your own sparklines by using `#plot=done` or `#plot=new`, in combination with `#n=10` for sparkline size, `#since=7d` / `#until=1d` for specifying time window.
* overall completed subtasks are plotted on a sparkline at the top of the screen for the past week.
* you can draw arrows between nodes for mind-mapping functionality
* can shell out and execute the content of a node with C-k. if the node starts with txt: this will be opened in vim or an editor specified in the `EDITOR` env var.

## what I do (don't do what I do, discover what works for you)
* create a #task subtree
* create different story subtrees for life goals, projects, etc... and tag them, #climbing #reading #client_143 etc...
* set up graphs for feedback on different goals/projects. `#tagged=climbing #since=30d #plot=done`
* start the day by fiddling with `#prio=<n>` tags on the stories
* hit the auto-task keybind (by default `C-v`) to pick an incomplete task child from one of the stories
* work on it for 25 minutes or until completion, optionally leaving a few minutes for a retrospective/reprioritization at the end
* distract myself as much as possible, let brain GC whatever I've been thinking about a little bit
* if I've completed a task, mark it done (by default, `C-a`)
* completed work is surfaced in the sparkline graphs I've set up for its tags
* every week or so, tweak the system

#### install

`cargo install voidmap`

if you don't have cargo, an easy way to get and manage
it is [via rustup](https://www.rustup.rs/). Ensure that `~/.cargo/bin`
is in your `$PATH` afterward, so that you can use the `rustup` and `cargo`
commands.

If you get errors along the lines of ``error: the `?' operator is not stable`` then
you need to update your rust compiler. If you installed rust with rustup, this can
be accomplished with `rustup update`.  Requires a recent stable rust compiler,
`1.14.0` or higher is recommended. This can be checked with `rustc --version`.
If you have installed rust with rustup, but you have an old version, there may be
an older version previously installed on your system. Verify that `which cargo`
outputs a path that belongs to your `.cargo/bin` directory.

#### invocation

`void`

this attempts to use `$HOME/.void.db` as a storage file.
if you'd like to specify a different storage file:

`void [/path/to/savefile]`

#### keys

feature | control | feature | control
--- | --- | --- | ---
new node | C-n | new node (child of selected) | Tab
new node (freeform) | click blank space | new node (sibling of selected) | Enter
delete selected node and its children | Delete | move subtree | drag parent to new location
undo delete | C-z | auto arrange nodes in view | C-p
mark selected node complete | C-a | drill-down into selected node | C-w
pop up selection | C-q | hide children of selected | C-t
open text editor for `txt:...` node | C-k | prefix-jump with no selection | type a letter
prefix-jump with other selected | C-f | hide completed children of node | C-h
select arrow start/destination | C-r | erase arrow | select start, C-r, then destination, C-r
show debug log | C-l | reparent node | drag node to new parent
scroll up | PgUp | scroll down | PgDn
select up | Up | select down | Down
select subtree to left | Left | select subtree to right | Right
de-select node | Esc | save | C-x
exit | Esc with nothing selected | exit | C-c
jump to weighted next task | C-v | cut / paste node | C-y
move selected up in child list | C-g | move selected down in child list | C-d
search for node at or below current view | C-u

can be customized by setting the `KEYFILE` env var to the path of a [key configuration file](default.keys)

#### known bugs

doesn't properly handle very long text. if you want to embed
an essay, create a node that begins with `txt: ` and hit `C-k`
to open its contents in an external text editor, specifiable
by setting the `EDITOR` env var.

#### optional configuration

setting the `LOGFILE` environment variable will allow you to
log debugging info to a file.

setting the `EDITOR` environment variable will allow you to
specify which text editor is opened when hitting `C-k` on a
node whose name begins with `txt: `.  defaults to vim.

setting the `KEYFILE` environment variable to the path of a
[keyfile](default.keys) allows you to customize the controls

setting the `LOCATION_QUERY` environment variable to anything
will enable an http request that is sent out at startup to
get approximate latitude and longitude coordinates associated
with your internet-facing IP. this is added to any nodes created
during a session, and eventually will allow you to trace the
rough path you've taken over time. aimed mostly at users who
travel a lot, may eventually have a more interesting implementation.

#### notes

This came about in the midst of an (ongoing) obsessive inquiry into a
cluster of topics roughly related to "effectiveness" while stumbling
through various mountain ranges and cities in central europe and the
american northeast.

* conversations with [@matthiasn](https://github.com/matthiasn) and being introduced
to his wonderful [iWasWhere](https://github.com/matthiasn/iWasWhere) system
* [writings of eliezer s. yudkowsky](https://wiki.lesswrong.com/wiki/Rationality:_From_AI_to_Zombies),
[how to solve it](https://en.wikipedia.org/wiki/How_to_Solve_It),
[society of mind](http://www.acad.bg/ebook/ml/Society%20of%20Mind.pdf)
* [various subtopics of operations research](https://en.wikipedia.org/wiki/Operations_research#Problems_addressed)
* occult assumption confrontation: [undoing yourself with energized meditation]
(http://heruka.altervista.org/X_files/Undoing%20Yourself%20With%20Energized%20Meditation%20And%20Other%20Devices%20-%20Christopher%20S%20Hyatt.pdf),
[prometheus rising](http://www.principiadiscordia.com/downloads/04%20Prometheus%20Rising.pdf)
* military C2 theory, recognition/metacognition, OODA, etc... [A Review of Time Critical Decision Making Models and
Human Cognitive Processes](https://pdfs.semanticscholar.org/2eb9/e12955dfafd4ab5d9337b416e31f5afca834.pdf)
* personal productivity literature: [pomodoro](http://baomee.info/pdf/technique/1.pdf), [GTD](https://en.wikipedia.org/wiki/Getting_Things_Done),
[eat that frog](http://www.actnow.ie/files/BookSummaryEatThatFrog.pdf), [flow](http://216.119.127.164/edgeware/archive/think/main_filing15.htm)

> The primary thing when you take a sword in your
hands is your intention to cut the enemy, whatever
the means. Whenever you parry, hit, spring, strike
or touch the enemy’s cutting sword, you must cut
the enemy in the same movement. It is essential to
attain this. If you think only of hitting, springing,
striking or touching the enemy, you will not be able
actually to cut him. More than anything, you must
be thinking of carrying your movement through to
cutting him... When you appreciate the power of
nature, knowing the rhythm of any situation, you
will be able to hit the enemy naturally and strike
naturally. All this is the Way of the Void. - Miyamoto Musashi


