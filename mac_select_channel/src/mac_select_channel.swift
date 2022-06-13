import Foundation
import CoreWLAN

@_cdecl("mac_select_channel")
//public func mac_select_channel() -> Int {
//public func mac_select_channel(_ name: String, _ numberChannel: Int) -> Int {
public func mac_select_channel(_ nameDev: UnsafePointer<CChar>, _ numberChannel: Int) -> Int {

//let numberChannel = 1
let name: String = String(utf8String: nameDev)!


//  var band: String
//  var width: String

  let widthChannel: CWChannelWidth = .width20MHz 
  let bandChannel: CWChannelBand = .band2GHz
  var succesSelector: Int = 0

  let interface = CWWiFiClient.shared().interface(withName: name)

  if interface != nil {

      do {try interface!.setPower(true)
      } catch {
        succesSelector = 0 //"No set power"
      }

    let channels = interface!.supportedWLANChannels()

    if channels != nil {
      for channel in channels! {

        if channel.channelNumber == numberChannel &&
          channel.channelWidth == widthChannel &&
          channel.channelBand == bandChannel {

          interface!.disassociate()

          do { try interface!.setWLANChannel(channel)
            succesSelector = 1 //"Ok"
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
  return succesSelector
}

//let succes = selectorChannel(name, numberChannel)
//print(succes)
