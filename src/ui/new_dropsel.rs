use super::network;
use gtk::prelude::*;
use gtk::Orientation;

pub fn newmediadropsel(playbackinfo: network::Media) -> gtk::Box {
    let vbox = gtk::Box::new(Orientation::Vertical, 5);
    let namelist = gtk::StringList::new(&[]);
    let audiolist = gtk::StringList::new(&[]);

    let sublist = gtk::StringList::new(&[]);

    let mut _set = 1;
    for media in playbackinfo.MediaSources.clone() {
        namelist.append(&media.Name);
        if _set == 1 {
            for stream in media.MediaStreams {
                if stream.Type == "Audio" {
                    if let Some(s) = stream.DisplayTitle {
                        audiolist.append(&s);
                    } else {
                        println!("No value");
                    }
                } else if stream.Type == "Subtitle" {
                    if let Some(d) = stream.DisplayTitle {
                        sublist.append(&d);
                    } else {
                        println!("No value");
                    }
                }
            }
        }
        _set = 0;
    }

    let namedropdown = gtk::DropDown::new(Some(namelist), Option::<gtk::Expression>::None);
    let audiodropdown = gtk::DropDown::new(Some(audiolist.clone()), Option::<gtk::Expression>::None);
    let subdropdown = gtk::DropDown::new(Some(sublist.clone()), Option::<gtk::Expression>::None);

    let playback_info = playbackinfo.clone();

    namedropdown.connect_selected_item_notify(move |dropdown| {
        let selected = dropdown.selected_item();
        let selected = selected.and_downcast_ref::<gtk::StringObject>().unwrap();
        let selected = selected.string();
        for _i in 1..audiolist.n_items() {
            audiolist.remove(0);
        }
        for _i in 1..sublist.n_items() {
            sublist.remove(0);
        }
        for media in playbackinfo.MediaSources.clone() {
            if media.Name == selected {
                for stream in media.MediaStreams {
                    if stream.Type == "Audio" {
                        if let Some(s) = stream.DisplayTitle {
                            audiolist.append(&s);
                        } else {
                            println!("No value");
                        }
                    } else if stream.Type == "Subtitle" {
                        if let Some(d) = stream.DisplayTitle {
                            sublist.append(&d);
                        } else {
                            println!("No value");
                        }
                    }
                }
                break;
            }
        }
    });
    vbox.append(&namedropdown);
    vbox.append(&audiodropdown);
    vbox.append(&subdropdown);
    

    let playbutton = gtk::Button::with_label("播放");
    playbutton.connect_clicked(move |_| {
        let nameselected = namedropdown.selected_item();
        let nameselected = nameselected.and_downcast_ref::<gtk::StringObject>().unwrap();
        let nameselected = nameselected.string();
        let subselected = subdropdown.selected_item();
        let subselected = subselected.and_downcast_ref::<gtk::StringObject>().unwrap();
        let subselected = subselected.string();
        for media in playback_info.MediaSources.clone() {
            if media.Name == nameselected {
                for mediastream in media.MediaStreams {
                    if mediastream.Type == "Subtitle" {
                        let displaytitle = mediastream.DisplayTitle.unwrap_or("".to_string());
                        if displaytitle == subselected {
                            let directurl = media.DirectStreamUrl.clone();
                            if mediastream.IsExternal == true {
                                let suburl = mediastream.DeliveryUrl.clone();
                                network::mpv_play_withsub(directurl.expect("no url"),suburl.expect("no url"),media.Name.clone());
                            } else {
                                network::mpv_play(directurl.expect("no url"),media.Name.clone());
                            }
                        }
                    }
                }
            }
        }
    });

    vbox.append(&playbutton);
    vbox
}