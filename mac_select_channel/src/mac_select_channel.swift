import Foundation
import CoreWLAN

@_cdecl("mac_select_channel")
public func mac_select_channel(_ nameDev: UnsafePointer<CChar>, _ numberChannel: Int) -> Int {

let name: String = String(utf8String: nameDev)!

  let widthChannel: CWChannelWidth = .width20MHz 
  let bandChannel: CWChannelBand = .band2GHz
  var succesSelector: Int = 0

  let interface = CWWiFiClient.shared().interface(withName: name)

  if interface != nil {

    do {try interface!.setPower(true)
    } catch {
      succesSelector = 1 //"No set power"
    }

    let channels = interface!.supportedWLANChannels()

    if channels != nil {
      for channel in channels! {

        if channel.channelNumber == numberChannel &&
          channel.channelWidth == widthChannel &&
          channel.channelBand == bandChannel {

          interface!.disassociate()
//          sleep(1)

          do { try interface!.setWLANChannel(channel)
            succesSelector = 0 //"Ok"
          }
          catch { 
            succesSelector = 2 //"No set channel"
          }
          break

        } else {
          succesSelector = 3 //"No selected channel"
        }
      }
    } else {
      succesSelector = 4 //"No list channels"
    }
  } else {
    succesSelector = 5 //"No interface"
  }
//  sleep(3)
  return succesSelector
}

@_cdecl("mac_get_current_channel")
func mac_get_current_channel(_ nameDev: UnsafePointer<CChar>) -> Int {
  let name: String = String(utf8String: nameDev)!
  var current_channel_number: Int = 0

let interface = CWWiFiClient.shared().interface(withName: name)
  if interface != nil {

      do {try interface!.setPower(true)
      } catch {
        current_channel_number = 0 //"No set power"
      }

    let current_channel = interface!.wlanChannel()
    if current_channel == nil {
      current_channel_number = 0 //"No set power on"
    } else {
      current_channel_number = current_channel!.channelNumber
    }
  } else {
    current_channel_number = -1 //"No interface"
  }
  return current_channel_number
}
