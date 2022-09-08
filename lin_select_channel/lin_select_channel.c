#include <stdlib.h>
#include <stdio.h>
#include <memory.h>
#include <linux/wireless.h>
#include <netinet/in.h>
#include <sys/ioctl.h>
#include <net/ethernet.h>

// char *device = "wlp3s0";
// int channel_new_freq = 2412;

int lin_select_channel(char *device, int channel_new_freq);
int lin_get_channel(char *device);

static int ioctlfd;
static int success;
static int channel_cur_freq;
static struct ifreq ifr;
static struct iwreq iwr;

static void ioctl_init(void);
static void ioctl_off(const char *device);
static void ioctl_monitor(const char *device);
static void ioctl_up(const char *device);
static void ioctl_get_channel(const char *device);
static void ioctl_set_channel(const char *device, int channel_new_freq);
static void ioctl_end(void);

int lin_select_channel(char *device, int channel_new_freq) {
    
    ioctl_init();
    if (ioctlfd < 0) {
        return 1;               // failed creation socket
    }

    ioctl_off(device);
    if (success != 0) {
        ioctl_end();
        return success;
    }

    ioctl_monitor(device);
    if (success != 0) {
        ioctl_end();
        return success;
    }

    ioctl_up(device);
    if (success != 0) {
        ioctl_end();
        return success;
    }

    ioctl_get_channel(device);
    if (success != 0) {
        ioctl_end();
        return success;
    }

    ioctl_set_channel(device, channel_new_freq);
    if (success != 0) {
        ioctl_end();
        return success;
    }

    ioctl_get_channel(device);
    if (success != 0) {
        ioctl_end();
        return success;
    }

    if(channel_new_freq != channel_cur_freq) {
        success = 9;                            // failed select freq channel driver
        ioctl_end();
        return success; 
    } else {
        ioctl_end();
        return success;
    }
}

int lin_get_channel(char *device) {
    
    ioctl_init();
    if (ioctlfd < 0) {
    return 1;               // failed creation socket
    }

    ioctl_get_channel(device);
    if (success != 0) {
        ioctl_end();
        return success;
    } else {
        ioctl_end();
        return channel_cur_freq;
    }   

}

/*===========================================================================*/

static void ioctl_init(void) {
    ioctlfd = socket(PF_PACKET, SOCK_RAW, htons(ETH_P_ALL));
}

static void ioctl_off(const char *device) {
    memset(&ifr, 0, sizeof(ifr));
    memcpy(&ifr.ifr_name, device, IFNAMSIZ);
    ifr.ifr_flags = 0;
    if(ioctl(ioctlfd, SIOCSIFFLAGS, &ifr) < 0) {
        success = 2;                    //failed DOWN device
        return;
    }
}

static void ioctl_monitor(const char *device) {
    memset(&iwr, 0, sizeof(iwr));
    memcpy(&iwr.ifr_name, device, IFNAMSIZ);
    iwr.u.mode = IW_MODE_MONITOR;
    if(ioctl(ioctlfd,SIOCSIWMODE, &iwr) <0) {
        success = 3;                    // failed set monitor mode
        return;
    }
}

static void ioctl_up(const char *device) {
    memset(&ifr, 0, sizeof(ifr));
    memcpy(&ifr.ifr_name, device, IFNAMSIZ);
    ifr.ifr_flags = IFF_UP;
    if(ioctl(ioctlfd, SIOCSIFFLAGS, &ifr) < 0) {
        success = 4;                    // failed UP device
        return;
    }
}

static void ioctl_get_channel(const char *device) {
    memset(&iwr, 0, sizeof(iwr));
    memcpy(&iwr.ifr_name, device, IFNAMSIZ);
    if(ioctl(ioctlfd, SIOCGIWFREQ, &iwr) < 0) {
        success = 5;                    // failed get current freq channel
        return;
    } else {
        if(iwr.u.freq.e == 6) channel_cur_freq = iwr.u.freq.m;
        else if(iwr.u.freq.e == 5) channel_cur_freq = iwr.u.freq.m /10;
        else if(iwr.u.freq.e == 4) channel_cur_freq = iwr.u.freq.m /100;
        else if(iwr.u.freq.e == 3) channel_cur_freq = iwr.u.freq.m /1000;
        else if(iwr.u.freq.e == 2) channel_cur_freq = iwr.u.freq.m /10000;
        else if(iwr.u.freq.e == 1) channel_cur_freq = iwr.u.freq.m /100000;
        else if(iwr.u.freq.e == 0) channel_cur_freq = iwr.u.freq.m /1000000;
        else {
            success = 6;                    // driver doesn't report freq
            return;
        }
    }
}

static void ioctl_set_channel(const char *device, int channel_new_freq) {
    memset(&iwr, 0, sizeof(iwr));
    memcpy(&iwr.ifr_name, device, IFNAMSIZ);
    iwr.u.freq.flags = IW_FREQ_FIXED;
    iwr.u.freq.e = 6;
    if(channel_cur_freq > channel_new_freq) {
        for (; channel_cur_freq >= channel_new_freq; channel_cur_freq -= 5) {
            iwr.u.freq.m = channel_cur_freq;
            if(ioctl(ioctlfd, SIOCSIWFREQ, &iwr) < 0) {
                success = 7;                // failed pass new freq channel down to driver
                return;
            }
        }
    }
    else if(channel_cur_freq < channel_new_freq) {
        for (; channel_cur_freq <= channel_new_freq; channel_cur_freq += 5) {
            iwr.u.freq.m = channel_cur_freq;
            if(ioctl(ioctlfd, SIOCSIWFREQ, &iwr) < 0) {
                success = 8;                // failed pass new freq channel up to driver
                return;
            }
        }
    } else return;
}

static void ioctl_end(void) {
    shutdown(ioctlfd, SHUT_RDWR);
    ioctlfd = -1;
}
