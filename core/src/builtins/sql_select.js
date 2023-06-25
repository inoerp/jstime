// sqlSelect
// https://html.spec.whatwg.org/multipage/timers-and-user-prompts.html#microtask-queuing

'use strict';

// eslint-disable-next-line no-unused-expressions
(({ sqlSelect }) => {
  globalThis.sqlSelect = (args) => {
    console.log("In sql select js " + JSON.stringify(args));
    return "5656";
  };
});
