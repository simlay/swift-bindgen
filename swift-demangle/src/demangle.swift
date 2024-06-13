import Darwin
import Foundation

typealias Swift_Demangle = @convention(c) (_ mangledName: UnsafePointer<UInt8>?,
                                           _ mangledNameLength: Int,
                                           _ outputBuffer: UnsafeMutablePointer<UInt8>?,
                                           _ outputBufferSize: UnsafeMutablePointer<Int>?,
                                           _ flags: UInt32) -> UnsafeMutablePointer<Int8>?

func swift_demangle(_ mangled: String) -> String? {
    let RTLD_DEFAULT = dlopen(nil, RTLD_NOW)
    if let sym = dlsym(RTLD_DEFAULT, "swift_demangle") {
        let f = unsafeBitCast(sym, to: Swift_Demangle.self)
        if let cString = f(mangled, mangled.count, nil, nil, 0) {
            defer { cString.deallocate() }
            return String(cString: cString)
        }
    }
    return nil
}

@_cdecl("swift_demangle")
func ffi_swift_demangle(_ mangled: UnsafePointer<CChar>) -> UnsafePointer<CChar>? {
    let mangled  = String(cString: UnsafePointer<CChar>(mangled))

    if let demangled = swift_demangle(mangled) {
        return (demangled as NSString).utf8String
    } else {
        return nil
    }
}

