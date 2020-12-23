# clusterctl

This is a rust reimplementation of the [clusterctrl][clusterctrl] python script,
to work in conjunction with the [ClusterHAT][clusterhat].

[clusterctrl]: https://github.com/burtyb/clusterhat-image/blob/master/files/sbin/clusterctrl
[clusterhat]: https://clusterhat.com/

## Usage

Get the power status of the Pis attached to the ClusterHAT.

	$ clusterctl status

Turn on the raspberrypi in slot 1.

	$ clusterctl on 1

Turn off the raspberrypi in slot 2.

	$ clusterctl off 2
