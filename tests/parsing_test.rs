mod tests {
    use quick_protobuf::{BytesReader, MessageRead};
    use rusp::usp::{mod_Body::OneOfmsg_body::request, Msg};
    use rusp::usp_record::{mod_Record::OneOfrecord_type::no_session_context, Record};

    #[test]
    fn simple_notify() {
        let bytes: Vec<u8> = vec![
            0x0a, 0x03, 0x31, 0x2e, 0x30, 0x1a, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x3a, 0x3a,
            0x61, 0x78, 0x2d, 0x75, 0x73, 0x70, 0x2d, 0x61, 0x67, 0x65, 0x6e, 0x74, 0x2d, 0x6e,
            0x6f, 0x73, 0x73, 0x6c, 0x2d, 0x77, 0x65, 0x62, 0x73, 0x6f, 0x63, 0x6b, 0x65, 0x74,
            0x3a, 0x4d, 0x12, 0x4b, 0x0a, 0x08, 0x0a, 0x04, 0x74, 0x65, 0x73, 0x74, 0x10, 0x03,
            0x12, 0x3f, 0x0a, 0x3d, 0x42, 0x3b, 0x0a, 0x0f, 0x73, 0x75, 0x62, 0x73, 0x63, 0x72,
            0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x42, 0x28, 0x0a, 0x03, 0x6f,
            0x75, 0x69, 0x12, 0x0d, 0x70, 0x72, 0x6f, 0x64, 0x75, 0x63, 0x74, 0x5f, 0x63, 0x6c,
            0x61, 0x73, 0x73, 0x1a, 0x0d, 0x73, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x5f, 0x6e, 0x75,
            0x6d, 0x62, 0x65, 0x72, 0x22, 0x03, 0x31, 0x2e, 0x30,
        ];

        use rusp::usp::{
            mod_Header::MsgType::NOTIFY, mod_Notify::OneOfnotification::on_board_req,
            mod_Request::OneOfreq_type::notify,
        };

        let mut reader = BytesReader::from_bytes(&bytes);
        let record = Record::from_reader(&mut reader, &bytes).expect("Cannot read Record");

        assert!(record.version == "1.0");
        assert!(record.from_id == "proto::ax-usp-agent-nossl-websocket");
        assert!(record.payload_security == "".into());
        assert!(record.mac_signature == vec!());
        assert!(record.sender_cert == vec!());
        assert!(if let no_session_context(_) = record.record_type {
            true
        } else {
            false
        });

        if let no_session_context(context) = record.record_type {
            let mut reader = BytesReader::from_bytes(&context.payload);
            let msg = Msg::from_reader(&mut reader, &context.payload).expect("Cannot read Msg");
            assert!(msg.header.is_some());

            if let Some(header) = msg.header {
                assert!(header.msg_id == "test");
                assert!(header.msg_type == NOTIFY);
            }

            if let Some(body) = msg.body {
                if let request(req) = body.msg_body {
                    assert!(if let notify(_) = req.req_type {
                        true
                    } else {
                        false
                    });

                    if let notify(a) = req.req_type {
                        assert!(a.subscription_id == "subscription_id");
                        assert!(a.send_resp == false);
                        if let on_board_req(n) = a.notification {
                            assert!(n.oui == "oui");
                            assert!(n.product_class == "product_class");
                            assert!(n.serial_number == "serial_number");
                            assert!(n.agent_supported_protocol_versions == "1.0");
                        }
                    }
                }
            }
        };
    }

    #[test]
    fn create_obj() {
        let bytes: Vec<u8> = vec![
            0x0a, 0x03, 0x31, 0x2e, 0x30, 0x12, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x3a, 0x3a,
            0x61, 0x78, 0x2d, 0x75, 0x73, 0x70, 0x2d, 0x61, 0x67, 0x65, 0x6e, 0x74, 0x2d, 0x6e,
            0x6f, 0x73, 0x73, 0x6c, 0x2d, 0x77, 0x65, 0x62, 0x73, 0x6f, 0x63, 0x6b, 0x65, 0x74,
            0x1a, 0x1e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x3a, 0x3a, 0x61, 0x78, 0x2d, 0x75, 0x73,
            0x70, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x2d, 0x6e,
            0x6f, 0x73, 0x73, 0x6c, 0x3a, 0x6e, 0x12, 0x6c, 0x0a, 0x1a, 0x0a, 0x16, 0x41, 0x58,
            0x53, 0x53, 0x2d, 0x31, 0x35, 0x34, 0x34, 0x31, 0x31, 0x34, 0x30, 0x38, 0x33, 0x2e,
            0x37, 0x36, 0x31, 0x35, 0x30, 0x38, 0x10, 0x08, 0x12, 0x4e, 0x0a, 0x4c, 0x2a, 0x4a,
            0x08, 0x01, 0x12, 0x46, 0x0a, 0x1d, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x4c,
            0x6f, 0x63, 0x61, 0x6c, 0x41, 0x67, 0x65, 0x6e, 0x74, 0x2e, 0x43, 0x6f, 0x6e, 0x74,
            0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x2e, 0x12, 0x0f, 0x0a, 0x05, 0x41, 0x6c, 0x69,
            0x61, 0x73, 0x12, 0x04, 0x74, 0x65, 0x73, 0x74, 0x18, 0x01, 0x12, 0x14, 0x0a, 0x0a,
            0x45, 0x6e, 0x64, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x49, 0x44, 0x12, 0x04, 0x74, 0x65,
            0x73, 0x74, 0x18, 0x01,
        ];

        use rusp::usp::{mod_Header::MsgType::ADD, mod_Request::OneOfreq_type::add};

        let mut reader = BytesReader::from_bytes(&bytes);
        let record = Record::from_reader(&mut reader, &bytes).expect("Cannot read Record");

        assert!(record.version == "1.0");
        assert!(record.to_id == "proto::ax-usp-agent-nossl-websocket");
        assert!(record.from_id == "proto::ax-usp-controller-nossl");
        assert!(record.payload_security == "".into());
        assert!(record.mac_signature == vec!());
        assert!(record.sender_cert == vec!());
        assert!(if let no_session_context(_) = record.record_type {
            true
        } else {
            false
        });

        if let no_session_context(context) = record.record_type {
            let mut reader = BytesReader::from_bytes(&context.payload);
            let msg = Msg::from_reader(&mut reader, &context.payload).expect("Cannot read Msg");
            assert!(msg.header.is_some());

            if let Some(header) = msg.header {
                assert!(header.msg_id == "AXSS-1544114083.761508");
                assert!(header.msg_type == ADD);
            }

            if let Some(body) = msg.body {
                if let request(req) = body.msg_body {
                    assert!(if let add(_) = req.req_type {
                        true
                    } else {
                        false
                    });

                    if let add(a) = req.req_type {
                        assert!(a.allow_partial == true);
                        assert!(a.create_objs.len() == 1);
                        let createobj = &a.create_objs[0];
                        assert!(createobj.obj_path == "Device.LocalAgent.Controller.");
                        assert!(createobj.param_settings.len() == 2);
                        let param1 = &createobj.param_settings[0];
                        assert!(param1.param == "Alias");
                        assert!(param1.value == "test");
                        assert!(param1.required == true);

                        let param2 = &createobj.param_settings[1];
                        assert!(param2.param == "EndpointID");
                        assert!(param2.value == "test");
                        assert!(param2.required == true);
                    }
                }
            }
        };
    }

    #[test]
    fn delete_obj() {
        let bytes: Vec<u8> = vec![
            0x0a, 0x03, 0x31, 0x2e, 0x30, 0x12, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x3a, 0x3a,
            0x61, 0x78, 0x2d, 0x75, 0x73, 0x70, 0x2d, 0x61, 0x67, 0x65, 0x6e, 0x74, 0x2d, 0x6e,
            0x6f, 0x73, 0x73, 0x6c, 0x2d, 0x77, 0x65, 0x62, 0x73, 0x6f, 0x63, 0x6b, 0x65, 0x74,
            0x1a, 0x1e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x3a, 0x3a, 0x61, 0x78, 0x2d, 0x75, 0x73,
            0x70, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x2d, 0x6e,
            0x6f, 0x73, 0x73, 0x6c, 0x3a, 0x4a, 0x12, 0x48, 0x0a, 0x1a, 0x0a, 0x16, 0x41, 0x58,
            0x53, 0x53, 0x2d, 0x31, 0x35, 0x34, 0x34, 0x31, 0x31, 0x34, 0x31, 0x30, 0x32, 0x2e,
            0x36, 0x36, 0x38, 0x34, 0x33, 0x39, 0x10, 0x0a, 0x12, 0x2a, 0x0a, 0x28, 0x32, 0x26,
            0x08, 0x01, 0x12, 0x22, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x4c, 0x6f, 0x63,
            0x61, 0x6c, 0x41, 0x67, 0x65, 0x6e, 0x74, 0x2e, 0x4d, 0x54, 0x50, 0x2e, 0x31, 0x2e,
            0x57, 0x65, 0x62, 0x53, 0x6f, 0x63, 0x6b, 0x65, 0x74, 0x2e,
        ];

        use rusp::usp::{mod_Header::MsgType::DELETE, mod_Request::OneOfreq_type::delete};

        let mut reader = BytesReader::from_bytes(&bytes);
        let record = Record::from_reader(&mut reader, &bytes).expect("Cannot read Record");

        assert!(record.version == "1.0");
        assert!(record.to_id == "proto::ax-usp-agent-nossl-websocket");
        assert!(record.from_id == "proto::ax-usp-controller-nossl");
        assert!(record.payload_security == "".into());
        assert!(record.mac_signature == vec!());
        assert!(record.sender_cert == vec!());
        assert!(if let no_session_context(_) = record.record_type {
            true
        } else {
            false
        });

        if let no_session_context(context) = record.record_type {
            let mut reader = BytesReader::from_bytes(&context.payload);
            let msg = Msg::from_reader(&mut reader, &context.payload).expect("Cannot read Msg");
            assert!(msg.header.is_some());

            if let Some(header) = msg.header {
                assert!(header.msg_id == "AXSS-1544114102.668439");
                assert!(header.msg_type == DELETE);
            }

            if let Some(body) = msg.body {
                if let request(req) = body.msg_body {
                    assert!(if let delete(_) = req.req_type {
                        true
                    } else {
                        false
                    });

                    if let delete(a) = req.req_type {
                        assert!(a.allow_partial == true);
                        assert!(a.obj_paths.len() == 1);
                        assert!(a.obj_paths[0] == "Device.LocalAgent.MTP.1.WebSocket.");
                    }
                }
            }
        };
    }

    #[test]
    fn get_obj() {
        let bytes: Vec<u8> = vec![
            0x0a, 0x03, 0x31, 0x2e, 0x30, 0x12, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x3a, 0x3a,
            0x61, 0x78, 0x2d, 0x75, 0x73, 0x70, 0x2d, 0x61, 0x67, 0x65, 0x6e, 0x74, 0x2d, 0x6e,
            0x6f, 0x73, 0x73, 0x6c, 0x2d, 0x77, 0x65, 0x62, 0x73, 0x6f, 0x63, 0x6b, 0x65, 0x74,
            0x1a, 0x1e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x3a, 0x3a, 0x61, 0x78, 0x2d, 0x75, 0x73,
            0x70, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x2d, 0x6e,
            0x6f, 0x73, 0x73, 0x6c, 0x3a, 0x48, 0x12, 0x46, 0x0a, 0x1a, 0x0a, 0x16, 0x41, 0x58,
            0x53, 0x53, 0x2d, 0x31, 0x35, 0x34, 0x34, 0x31, 0x31, 0x34, 0x30, 0x34, 0x35, 0x2e,
            0x34, 0x34, 0x32, 0x35, 0x39, 0x36, 0x10, 0x01, 0x12, 0x28, 0x0a, 0x26, 0x0a, 0x24,
            0x0a, 0x22, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x4c, 0x6f, 0x63, 0x61, 0x6c,
            0x41, 0x67, 0x65, 0x6e, 0x74, 0x2e, 0x4d, 0x54, 0x50, 0x2e, 0x31, 0x2e, 0x57, 0x65,
            0x62, 0x53, 0x6f, 0x63, 0x6b, 0x65, 0x74, 0x2e,
        ];

        use rusp::usp::{mod_Header::MsgType::GET, mod_Request::OneOfreq_type::get};

        let mut reader = BytesReader::from_bytes(&bytes);
        let record = Record::from_reader(&mut reader, &bytes).expect("Cannot read Record");

        assert!(record.version == "1.0");
        assert!(record.to_id == "proto::ax-usp-agent-nossl-websocket");
        assert!(record.from_id == "proto::ax-usp-controller-nossl");
        assert!(record.payload_security == "".into());
        assert!(record.mac_signature == vec!());
        assert!(record.sender_cert == vec!());
        assert!(if let no_session_context(_) = record.record_type {
            true
        } else {
            false
        });

        if let no_session_context(context) = record.record_type {
            let mut reader = BytesReader::from_bytes(dbg!(&context.payload));
            let msg = Msg::from_reader(&mut reader, &context.payload).expect("Cannot read Msg");
            assert!(msg.header.is_some());

            if let Some(header) = msg.header {
                assert!(header.msg_id == "AXSS-1544114045.442596");
                assert!(header.msg_type == GET);
            }

            if let Some(body) = msg.body {
                if let request(req) = body.msg_body {
                    assert!(if let get(_) = req.req_type {
                        true
                    } else {
                        false
                    });

                    if let get(a) = req.req_type {
                        assert!(a.param_paths.len() == 1);
                        assert!(a.param_paths[0] == "Device.LocalAgent.MTP.1.WebSocket.");
                    }
                }
            }
        };
    }
}
