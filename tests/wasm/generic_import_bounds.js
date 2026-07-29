exports.echo = function(x) {
  return x;
};

exports.sum = function(a, b) {
  return Number(a) + Number(b);
};

let log = [];

exports.record = function(x) {
  log.push(String(x));
};

exports.takeLog = function() {
  const s = log.join(",");
  log = [];
  return s;
};
