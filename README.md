*Backup Manager*

A TCP Daemon that accepts pre-defined commands to execute locally. In 
principle these commands can be anything, but it is written with as a backup manager.
Set predefined rsync commands in the backup-manager.conf and start the daemon. 

When the "start mybackup1" command is written to port 9123 the daemon executes the mybackup1
command as it is defined in the backup-manager.conf file.
