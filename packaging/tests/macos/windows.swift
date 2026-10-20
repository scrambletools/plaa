// The test's eyes on the screen, built by test-pkg.sh beside it:
//
//   windows list        each window on screen: its number, owner and name
//   windows bars PNG    the fewest pixels of any of the yellow, cyan and
//                       magenta bars of the test clip in a picture, in
//                       hundredths of the picture
import CoreGraphics
import Foundation
import ImageIO

let arguments = CommandLine.arguments
switch arguments.count > 1 ? arguments[1] : "" {
case "list":
    let options: CGWindowListOption = [.optionOnScreenOnly, .excludeDesktopElements]
    for window in CGWindowListCopyWindowInfo(options, kCGNullWindowID) as? [[String: Any]] ?? [] {
        let number = window[kCGWindowNumber as String] as? Int ?? 0
        let owner = window[kCGWindowOwnerName as String] as? String ?? ""
        let name = window[kCGWindowName as String] as? String ?? ""
        print("\(number)\t\(owner)\t\(name)")
    }
case "bars" where arguments.count > 2:
    guard let source = CGImageSourceCreateWithURL(URL(fileURLWithPath: arguments[2]) as CFURL, nil),
        let image = CGImageSourceCreateImageAtIndex(source, 0, nil)
    else {
        print(0)
        exit(0)
    }
    let width = image.width, height = image.height
    var pixels = [UInt8](repeating: 0, count: width * height * 4)
    let drawn = pixels.withUnsafeMutableBytes { buffer -> Bool in
        guard let context = CGContext(
            data: buffer.baseAddress, width: width, height: height, bitsPerComponent: 8,
            bytesPerRow: width * 4, space: CGColorSpaceCreateDeviceRGB(),
            bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue)
        else { return false }
        context.draw(image, in: CGRect(x: 0, y: 0, width: width, height: height))
        return true
    }
    var yellow = 0, cyan = 0, magenta = 0
    if drawn {
        for at in stride(from: 0, to: pixels.count, by: 4) {
            let red = pixels[at], green = pixels[at + 1], blue = pixels[at + 2]
            if red > 180 && green > 180 && blue < 80 { yellow += 1 }
            else if red < 80 && green > 180 && blue > 180 { cyan += 1 }
            else if red > 180 && green < 80 && blue > 180 { magenta += 1 }
        }
    }
    print(min(yellow, cyan, magenta) * 10000 / max(width * height, 1))
default:
    FileHandle.standardError.write("usage: windows list | windows bars PNG\n".data(using: .utf8)!)
    exit(2)
}
