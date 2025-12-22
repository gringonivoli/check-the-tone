export CURRENT_BRANCH:=`git symbolic-ref --short -q HEAD`

default:
    just --list

# pull/push to repo sync
sync:
    ./.scripts/pull-push.sh

# keep the repo sync looping pull/push
keep-sync:
    ./.scripts/watch.sh

test:
    cargo test

test-e2e:
    cargo test -- --ignored

test-all:
    cargo test -- --include-ignored

# run tests when a file is modified
test-watch:
    bacon test

coverage:
    cargo llvm-cov test --html --open

coverage-all:
    cargo llvm-cov test --html --open -- --include-ignored
